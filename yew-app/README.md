## Yew 是什么？

[YEW官网](https://yew.rs/zh-CN)

**Yew** 是一个设计先进的 [Rust](https://www.rust-lang.org/) 框架，目的是使用 [WebAssembly](https://webassembly.org/) 来创建多线程的前端 web 应用。

-   **基于组件的框架**，可以轻松的创建交互式 UI。拥有 [React](https://reactjs.org/) 或 [Elm](https://elm-lang.org/) 等框架经验的开发人员在使用 Yew 时会感到得心应手。
-   **高性能** ，前端开发者可以轻易的将工作分流至后端来减少 DOM API 的调用，从而达到异常出色的性能。
-   **支持与 JavaScript 交互** ，允许开发者使用 NPM 包，并与现有的 JavaScript 应用程序结合。

# 第一个简单的 App

首先，创建一个新的 cargo 项目：

```
cargo new yew-app
```

进入刚刚创建的目录。

现在，让我们在`Cargo.toml`文件中添加`yew`作为依赖项：

```
[package]
name = "yew-app"
version = "0.1.0"
edition = "2018"

[dependencies]
# 你可以在此处查看最新版本: https://crates.io/crates/yew
yew = "0.18"
```


将以下代码复制到您的`src/main.rs`文件中：

```rust
use yew::prelude::*;

enum Msg {
    AddOne,
}

struct Model {
    // `ComponentLink` is like a reference to a component.
    // It can be used to send messages to the component
    link: ComponentLink<Self>,
    value: i64,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link, value: 0 }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::AddOne => {
                self.value += 1;
                // the value has changed so we need to
                // re-render for it to appear on the page
                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        // Should only return "true" if new properties are different to
        // previously received properties.
        // This component has no properties so we will always return "false".
        false
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <h2>{"你的第一个Rust App"}</h2>
                <span>{"点击: "}</span>
                <button onclick=self.link.callback(|_| Msg::AddOne)>{ "+1" }</button>
                <p>{ self.value }</p>
            </div>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}
```



这份代码通过一个 `Model` 构建了你的根 `组件`，它会显示一个按钮，当你点击按钮时，Model 将会更新自己的状态。特别注意 main() 中的 `yew::start_app::<Model>()`，它会启动你的应用并将其挂载到页面的 `<body>` 标签中。如果你想使用任何动态属性来启动应用程序，则可以使用 `yew::start_app_with_props::<Model>(..)`。

最后，将`index.html`文件添加当前应用的根目录下：

```html
<!DOCTYPE html>
<html>
  <head>
    <meta charset="utf-8" />
    <title>Yew App</title>
  </head>
</html>
```

Copy

## 运行你的应用程序!

如果尚未安装 [Trunk](https://github.com/thedodd/trunk)，请执行以下操作：

```
cargo install trunk wasm-bindgen-cli
```

将`wasm32-unknown-unknown`添加为编译目标。 如果还未安装，请使用 Rustup 运行以下指令：

```
rustup target add wasm32-unknown-unknown
```

现在，您所要做的就是运行以下命令：

```
trunk serve
```
这将启动一个开发服务器，该服务器在您每次进行更改时都会更新应用程序。

如果一切顺利打开 http://0.0.0.0:8080/  你将看到

![截屏2021-09-28 下午3.16.34.png](https://p9-juejin.byteimg.com/tos-cn-i-k3u1fbpfcp/da453f0ebd674092833a6bc91142b25d~tplv-k3u1fbpfcp-watermark.image?)
## 常见问题

-   Trunk 安装失败：

    确保你已经安装了 openssl 的开发包。例如，Ubuntu 上的 libssl-dev 或 Fedora 上的 openssl-devel。
