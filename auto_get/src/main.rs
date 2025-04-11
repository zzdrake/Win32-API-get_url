use clipboard::{ClipboardContext, ClipboardProvider};
use std::{fs::File, io::Write, process::Command, thread, time::Duration};
use uiautomation::{
    UIElement, UITreeWalker,
    core::UIAutomation,
    types::{TreeScope, UIProperty},
};

async fn launch_browser() -> Result<(), ()> {
    Command::new(r"your-browser-path")
        .spawn()
        .unwrap();

    Ok(())
}

async fn get_edit(
    walker: &UITreeWalker,
    ele: &UIElement,
    file_name: &str,
    question: &str,
) -> Result<String, ()> {
    let destop = dirs::desktop_dir().expect("无法获取桌面路径");
    let file_path = destop.join(format!("{}.txt", file_name));
    File::create(&file_path).expect("无法创建文件");

    if let Ok(edit) = walker.get_first_child(&ele) {
        let _ = edit.send_keys(&format!("{} {{enter}}", question), 0);
    }

    Ok(file_path.to_string_lossy().into_owned())
}

async fn find_copy_btn(automation: &UIAutomation) -> Result<String, ()> {
    let condition = automation
        .create_property_condition(UIProperty::Name, "复制".into(), None)
        .unwrap();

    let btn = automation
        .get_root_element()
        .unwrap()
        .find_first(TreeScope::Descendants, &condition)
        .expect("找到不到复制按钮");

    btn.click().unwrap();
    println!("已复制");

    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
    let content = ctx.get_contents().unwrap_or_default();

    Ok(content)
}

#[tokio::main]
async fn main() -> Result<(), ()> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        println!("请提供要保存的文件名以及要提问的问题");
        println!("用法: ./auto_get <文件名> <问题>");
    }

    let filename = &args[1];
    let question = &args[2..].join(" ");

    launch_browser().await?;

    let automation = UIAutomation::new().unwrap();
    let root = automation.get_root_element().unwrap();
    let walker = automation.get_content_view_walker().unwrap();

    thread::sleep(Duration::from_secs(1));

    root.send_keys("https://chatgpt.com/ {enter}", 0).unwrap();

    thread::sleep(Duration::from_secs(10));

    let file_path = get_edit(&walker, &root, &filename, &question).await?;

    thread::sleep(Duration::from_secs(10));

    let copied_content = find_copy_btn(&automation).await?;

    let mut file = std::fs::OpenOptions::new()
        .append(true)
        .open(&file_path)
        .expect("无法打开文件");

    writeln!(file, "{}", copied_content).expect("写入文件失败");
    println!("写入成功!");

    Ok(())
}
