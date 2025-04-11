import uiautomation as auto
import time

# 支持的浏览器窗口类名
BROWSERS = {
    'Chrome': 'Chrome_WidgetWin_1',
    'Edge': 'Chrome_WidgetWin_1',
    'Firefox': 'MozillaWindowClass',
}

# 存储上一次的地址栏内容
last_urls = {browser: None for browser in BROWSERS}

def get_browser_url(browser_name):
    class_name = BROWSERS.get(browser_name)
    if not class_name:
        return None

    window = auto.WindowControl(searchDepth=1, ClassName=class_name)
    if not window.Exists(0, 0):
        return None

    if browser_name in ['Chrome', 'Edge']:
        edit = window.EditControl(foundIndex=1)
    elif browser_name == 'Firefox':
        edit = window.Control(searchDepth=10, ControlType='EditControl')
    else:
        return None

    if not edit.Exists():
        return None

    try:
        value_pattern = edit.GetValuePattern()
        return value_pattern.Value
    except:
        return None

if __name__ == "__main__":
    print("开始循环监控浏览器地址栏...（按 Ctrl+C 退出）")
    try:
        while True:
            for browser in BROWSERS:
                url = get_browser_url(browser)
                if url and url != last_urls[browser]:
                    print(f"📎 [{browser}] 新地址: {url}")
                    last_urls[browser] = url
            time.sleep(1)  # 每秒检查一次
    except KeyboardInterrupt:
        print("\n监控已终止")
