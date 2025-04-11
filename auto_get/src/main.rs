use std::{process::Command, thread, time::Duration};
use uiautomation::{
    UIElement, UITreeWalker,
    actions::Window,
    controls::WindowControl,
    core::UIAutomation,
    processes::Process,
    types::{TreeScope, UIProperty},
};

async fn launch_browser() -> Result<(), ()> {
    Command::new(r"path-to-your-browser")
        .spawn()
        .unwrap();

    Ok(())
}

async fn get_edit(walker: &UITreeWalker, ele: &UIElement) -> Result<(), ()> {
    if let Ok(edit) = walker.get_first_child(&ele) {
        let _ = edit.send_keys("Hello {enter}", 0);
    }

    Ok(())
}

async fn find_copy_btn(automation: &UIAutomation) -> Result<(), ()> {
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

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), ()> {
    launch_browser().await?;

    let automation = UIAutomation::new().unwrap();
    let root = automation.get_root_element().unwrap();
    let walker = automation.get_content_view_walker().unwrap();

    thread::sleep(Duration::from_secs(1));

    root.send_keys("https://chatgpt.com/ {enter}", 0).unwrap();

    thread::sleep(Duration::from_secs(10));

    get_edit(&walker, &root).await?;

    thread::sleep(Duration::from_secs(10));

    find_copy_btn(&automation).await?;

    Process::create("notepad.exe").unwrap();
    let matcher = automation
        .create_matcher()
        .from(root)
        .timeout(10000)
        .classname("Notepad");
    if let Ok(notepad) = matcher.find_first() {
        println!(
            "Found: {} - {}",
            notepad.get_name().unwrap(),
            notepad.get_classname().unwrap()
        );

        notepad.send_keys("{ctrl}v", 0).unwrap();
        notepad.send_keys("{ctrl}s", 0).unwrap();

        let window: WindowControl = notepad.try_into().unwrap();
        window.maximize().unwrap();
    }

    Ok(())
}
