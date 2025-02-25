# jiascheduler

**简体中文** · [English](./README.md) · [Wiki](https://github.com/jiawesoft/jiascheduler/wiki/Install)

一个用 rust 编写的开源高性能，可扩展，动态配置的任务调度器，支持同时推送用户脚本到数以万计的实例运行，并实时收集执行的结果。

jiascheduler 执行脚本的节点不需要都在同一个网络，其内部设计了一个精巧的网络穿透模型可以用一个控制台管理不同子网的节点；举例，你可以在 https://jiascheduler.iwannay.cn 同时往腾讯云， 阿里云，亚马逊云推送脚本执行，当然你可以往家里的电脑部署脚本执行。

为了方便对节点进行管理，jiascheduler 同时提供了一个功能强大的 webssh 终端，支持多会话操作，分屏，上传，下载等。

Github 地址：https://github.com/jiawesoft/jiascheduler

## 架构图

![架构图](./assets/jiascheduler-arch.png)

## 快速开始

### [💖 jiascheduler 下载点击这里 💖 ](https://github.com/jiawesoft/jiascheduler/releases)

[https://jiascheduler.iwannay.cn](https://jiascheduler.iwannay.cn)
访客账号：guest 密码：guest

此时 guest 账号下并没有在线的节点，你可以自己部署 Agent，部署成功的 Agent 将自动接入 jiascheduler 在线控制台，你可以在控制台查看 Agent 的状态，执行脚本，查看执行结果。

```bash
# 仅使用作业调度能力
./jiascheduler-agent --comet-addr ws://115.159.194.153:3000 --assign-username guest --assign-password guest

# 使用作业调度能力和webssh能力
./jiascheduler-agent --comet-addr ws://115.159.194.153:3000 --assign-username guest --assign-password guest --ssh-user your_ssh_user --ssh-port 22 --ssh-password your_ssh_user_password --namespace home
```

如果你需要下线节点，只需要退出 Agent 即可

### 手动编译

1. 编译前端项目

```bash
# 克隆仓库
git clone https://github.com/jiawesoft/jiascheduler-ui.git
# 安装依赖
cd jiascheduler-ui
pnpm install
# 编译项目
pnpm build
# 编译完成后，将dist目录下的文件复制到jiascheduler的dist目录下
cp -r dist/* jiascheduler/dist/
```

2. 编译 jiascheduler

```bash
# 编译
cargo build -r --target x86_64-unknown-linux-musl
# 查看编译后的执行文件
ls target/x86_64-unknown-linux-musl/release
```

### 完整安装

1. 安装 jiascheduler-console

```bash
# Usage: jiascheduler-console [OPTIONS]

# Options:
#   -d, --debug                        if enable debug mode
#       --bind-addr <BIND_ADDR>        http server listen address, eg: "0.0.0.0:9090"
#       --config <FILE>                where to read config file, you can temporarily overwrite the configuration file using command-line parameters [default: ~/.jiascheduler/console.toml]
#   -h, --help                         Print help
#   -V, --version                      Print version

# 首次安装需要指定--bind-addr，服务启动后访问0.0.0.0:9090，进入安装界面，按提示完成安装
./jiascheduler-console --bind-addr 0.0.0.0:9090
```

2. 安装 jiaschduler-comet

```bash
# Usage: jiascheduler-comet [OPTIONS]

# Options:
#   -d, --debug            if enable debug mode
#   -b, --bind <BIND>      [default: 0.0.0.0:3000]
#   -r <REDIS_URL>         [default: redis://:wang@127.0.0.1]
#       --secret <SECRET>  [default: rYzBYE+cXbtdMg==]
#   -h, --help             Print help
#   -V, --version          Print version

# 设置comet监听地址，secret则采用默认值
./jiascheduler-comet --bind 0.0.0.0:3000
```

3. 安装 jiascheduler-agent

```bash
# Usage: jiascheduler-agent [OPTIONS]

# Options:
#   -d, --debug
#           If enable debug mode
#   -b, --bind <BIND>
#           [default: 0.0.0.0:3001]
#       --comet-addr <COMET_ADDR>
#           [default: ws://127.0.0.1:3000]
#       --output-dir <OUTPUT_DIR>
#           Directory for saving job execution logs [default: ./log]
#       --comet-secret <COMET_SECRET>
#           [default: rYzBYE+cXbtdMg==]
#   -n, --namespace <NAMESPACE>
#           [default: default]
#       --ssh-user <SSH_USER>
#           Set the login user of the instance for SSH remote connection
#       --ssh-password <SSH_PASSWORD>
#           Set the login user's password of the instance for SSH remote connection
#       --ssh-port <SSH_PORT>
#           Set the port of this instance for SSH remote connection
#       --assign-username <ASSIGN_USERNAME>
#           Assign this instance to a user and specify their username
#       --assign-password <ASSIGN_PASSWORD>
#           Assign this instance to a user and specify their password
#   -h, --help
#           Print help
#   -V, --version
#           Print version


# 使用作业调度能力和webssh能力
# ssh相关配置也可以不传，稍后可以在控制台直接配置
./jiascheduler-agent --comet-addr ws://115.159.194.153:3000 --assign-username guest --assign-password guest --ssh-user your_ssh_user --ssh-port 22 --ssh-password your_ssh_user_password --namespace home

```

### docker 部署

在docker-compose.yml同目录下创建.env文件，内容如下

```shell
WORKCONF=/data/jiascheduler
WORKDATA=/data/jiascheduler
```

console.toml在容器中默认路径为/root/.jiascheduler/console.toml，如果没有，则访问console页面，填写相关信息，在会自动创建

如果存在console.toml文件，访问console页面则直接跳到登录页面，参考配置如下，将以下内容保存为console.toml，放$WORKCONF/.jiascheduler目录下

```yml

debug = false
bind_addr = "0.0.0.0:9090"
api_url = ""
redis_url = "redis://default:3DGiuazc7wkAppV3@redis"
comet_secret = "rYzBYE+cXbtdMg=="
database_url = "mysql://root:kytHmeBR4Vg@mysql:3306/jiascheduler"

[encrypt]
private_key = "QGr0LLnFFt7mBFrfol2gy"

[admin]
username = "admin"
password = "qTQhiMiLCb"

```

执行 docker compose up -d 后访问0.0.0.0:9090，如提示invalid username，说明目前通过配置文件启动暂不支持自动创建用户，需要执行以下 sql 创建用户。（自动生成console.toml会自动创建用户）

```sql
INSERT INTO jiascheduler.`user` (user_id,username,nickname,is_root,role_id,salt,password,avatar,email,phone,gender,introduction,created_time,updated_time) VALUES
	 ('NDoFVL5BKj','admin','admin',1,1,'FDzVZNHWWr3mPd6JBVcZD','d733f3b2c0662a4ce0c0f83cda78f7f2','','','','male','','2025-02-24 20:07:03','2025-02-24 20:07:03');

INSERT INTO jiascheduler.`role` (name,info,is_admin,created_user,created_time,updated_time) VALUES
	 ('admin','System initialization administrator role, unable to delete',1,'admin','2025-02-24 20:07:03','2025-02-24 20:07:03');
```

docker参考配置如下

[docker-compose.yml](docker-compose.yml)

## 软件截图

<table style="border-collapse: collapse; border: 1px solid black;">
  <tr>
    <td style="padding: 5px;background-color:#fff;"><img src= "./assets/job-edit.png" alt="Jiascheduler job edit"   /></td>
    <td style="padding: 5px;background-color:#fff;"><img src= "./assets/run-list.png" alt="Jiascheduler run list"   /></td>
  </tr>

  <tr>
    <td style="padding: 5px;background-color:#fff;"><img src= "./assets/scheduler-history.png" alt="Jiascheduler scheduler history"   /></td>
    <td style="padding: 5px;background-color:#fff;"><img src= "./assets/scheduler-dashboard.png" alt="Jiascheduler scheduler dashboard"   /></td>
  </tr>

  <tr>
    <td style="padding: 5px;background-color:#fff;"><img src= "./assets/server.png" alt="Jiascheduler server"   /></td>
    <td style="padding: 5px;background-color:#fff;"><img src= "./assets/webssh.png" alt="Jiascheduler webssh"   /></td>
  </tr>

</table>

## 帮助视频

https://www.bilibili.com/video/BV19wzKYVEHL

## 请杯咖啡

**wechat:** cg1472580369

<img src="./assets/good.jpg" width="400px" />
