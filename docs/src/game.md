# Game 模块

包含游戏内操作相关内容。

## 函数

### systemMessage(message: String, color: Option<String>)

在右侧显示系统消息。

> message: 要显示的消息内容。
>
> color: 消息颜色，可以为以下值：
>
> - "blue" or "general"
> - "purple" or "primary"
>
> 默认颜色为 "blue"。

### sendMessage(message: String)

发送聊天消息。发送目标默认为当前聊天框选中的对象，不进行更改。

> message: 要发送的消息内容。
