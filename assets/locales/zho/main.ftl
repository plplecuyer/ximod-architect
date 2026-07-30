# XIMOD Architect - translation metadata
# @language = zho
# @font = Noto_Sans_SC/static/NotoSansSC-Regular.ttf
# @langname = 汉语
# @author = XIMOD Team

# XIMOD Architect - English Translations

# Application
app-title = XIMOD Architect
app-version = 版本 { $version }

# Status messages
status-ready = 已就绪
msg-save-success = FOMOD 已成功保存
msg-save-error = 保存 FOMOD 时出错
msg-export-success = 已创建分发存档（{ $count } 个文件）：{ $path }
msg-export-error = 创建分发存档时出错：{ $error }
msg-load-success = FOMOD 已成功加载
msg-load-error = 加载 FOMOD 失败
msg-merge-success = FOMOD 合并成功
msg-merge-error = 合并 FOMOD 失败
msg-no-root-selected = 请先选择一个根目录
msg-no-fomod-folder = 未找到“fomod”文件夹。要创建一个吗？
msg-file-outside-root = 文件位于根目录之外

# Menu - File
menu-file = 文件
menu-new = 新建
menu-open = 打开文件夹...
menu-open-file = 打开文件...
menu-save = 保存
menu-recent = 最近
menu-exit = 退出
menu-merge = 合并 FOMOD...
menu-export = 导出发行版存档...

# Menu - Options
menu-options = 选项
menu-settings = 设置
menu-pre-save-script = 保存前脚本...
menu-post-save-script = 保存后脚本...
menu-translation = 翻译...

# Menu - Help
menu-help = 帮助
menu-about = 关于

# Tabs
tab-info = 模组信息
tab-steps = 安装步骤
tab-required = 必需安装项
tab-conditional = 条件安装项

# Info Tab
label-workspace = 工作区
label-root-dir = 根目录：
label-mod-name = 模组名称：
label-author = 作者：
label-version = 版本：
label-game-name = 游戏名称：
label-category = 分类：
label-url = 网站网址：
label-header-image = 封面图片：
label-description = 描述：
placeholder-select-dir = （选择一个目录）
placeholder-select-game = （选择一款游戏）

# Steps Tab
label-step-name = 步骤名称：
label-group-name = 组名称：
label-group-type = 组类型：
label-plugin-name = 插件名称：
label-plugin-desc = 描述：
label-plugin-type = 默认类型：
label-plugin-image = 图片：
label-visibility = 可见性条件
label-operator = 运算符：

# Buttons
btn-browse = 浏览...
btn-clear = 清除
btn-add = 添加
btn-remove = 移除
btn-add-step = 新建步骤
btn-delete-step = 删除步骤
btn-add-group = 添加组
btn-remove-group = 移除组
btn-add-plugin = 添加插件
btn-remove-plugin = 移除插件
btn-add-file = 添加文件
btn-add-folder = 添加文件夹
btn-remove-file = 移除
btn-add-flag = 添加标记
btn-remove-flag = 移除标记
btn-add-condition = 添加条件
btn-remove-condition = 移除条件
btn-add-dependency = 添加依赖关系
btn-remove-dependency = 移除依赖关系
btn-add-pattern = 新建模式
btn-remove-pattern = 删除模式
btn-save = 保存
btn-cancel = 取消
btn-ok = 确定
btn-yes = 是
btn-no = 否

# Condition/Dependency Labels
label-flag-name = 标志名称：
label-flag-value = 值：
label-condition-type = 类型：
label-condition-name = 名称：
label-condition-value = 值：
label-dep-type = 依赖类型：
label-dep-name = 名称/文件：
label-dep-value = 值/状态：

# Files
label-source = 源文件
label-destination = 目标文件
label-priority = 优先级
label-file-type = 类型
label-files = 文件
label-dependencies = 依赖项

# Settings Dialog
settings-title = 设置
settings-tab-general = 常规
settings-tab-recent-files = 最近文件
settings-language = 语言：
settings-theme = 主题：
settings-font-size = 字体大小：
settings-replace-newlines = 处理描述中的换行符
settings-max-recent = 最近文件上限：
settings-window-width = 窗口宽度：
settings-window-height = 窗口高度：
settings-no-recent-files = 没有最近文件。

# Status messages for settings
status-settings-saved = 设置已成功保存

# About Dialog
about-title = 关于 XIMOD Architect
about-description = 一款用于 Bethesda 游戏模组的跨平台 FOMOD 安装程序生成工具。
about-license = 采用 MIT 许可证
about-copyright = © 2025-2026 XIMOD 团队
about-credit = Wenderer 原版工具的 Rust 移植版：

# Script Dialog
script-title = 编辑脚本
script-info = 脚本将在保存前或保存后执行。您可以使用以下宏：
script-macros = 可用宏：
macro-modname = $MODNAME$ - 模组名称
macro-modauthor = $MODAUTHOR$ - 作者名称
macro-modversion = $MODVERSION$ - 模组版本
macro-modroot = $MODROOT$ - 根目录路径
macro-date = $DATE$ - 当前日期（YYYY-MM-DD）
macro-time = $TIME$ - 当前时间（HH:MM:SS）
macro-random = $RANDOM$ - 随机数

# Plugin Dependencies
label-default-type = 默认类型：
label-pattern-type = 模式类型：
label-pattern-operator = 模式运算符：

# Conditional Files
label-pattern = 模式

# Validation Messages
validation-no-name = 必须填写模块名称
validation-no-steps = 至少需要一个步骤或必填文件
validation-empty-step = 步骤 { $num } 未命名
validation-empty-group = 步骤 { $step }，组 { $group } 未命名
validation-no-plugins = 步骤 { $step }，组 "{ $name }" 未配置插件

# File States
state-active = 活动
state-inactive = 非活动
state-missing = 缺失

# Confirmation
confirm-title = 确认
confirm-delete = 您确定要删除此项目吗？
confirm-discard = 您有未保存的更改。是否放弃这些更改并继续？
confirm-unsaved = 您有未保存的更改。是否要在关闭前保存？
confirm-save-issues = 该项目存在以下问题：
confirm-save-anyway = 仍要保存吗？

# Errors
error-invalid-xml = XML 文件无效
error-parse-failed = 解析 FOMOD 失败
error-write-failed = 写入文件失败
error-create-dir = 创建目录失败

# Default names (generated when creating new items)
default-step-name = 步骤 { $num }
default-group-name = 组 { $num }
default-plugin-name = 插件 { $num }
pattern-label = 模式 { $num }

# Selection prompts
msg-select-group-first = 请先选择一个组。
msg-select-plugin-edit = 请选择要编辑的插件。
label-empty = (空)
image-no-image = 无图片

# File dialog filters
filter-images = 图片
filter-xml = XML

# Dependency types
dep-type-flag = 标志
dep-type-file = 文件

# Status bar
status-modified = 已修改

# Status messages (errors)
msg-settings-save-error = 保存设置时出错
msg-script-save-error = 保存脚本时出错

# Translation editor
trans-title = 翻译编辑器
trans-source-lang = 显示语言：
trans-target-lang = 目标语言：
trans-col-key = 键
trans-col-source = 标签
trans-col-target = 翻译
trans-saved = 翻译已保存
trans-save-error = 保存翻译时出错

# XML editor
xml-editor-title = XML 编辑器
xml-editor-edit = 编辑
xml-editor-apply = 应用
xml-editor-revert = 撤销
xml-editor-readonly = 只读
xml-editor-editing = 正在编辑 — 图形化选项卡已锁定
xml-editor-error = 错误：
xml-editor-applied = XML 更改已应用
xml-editor-wellformed = XML 格式正确
xml-editor-error-at = 第 { $line } 行，第 { $col } 列：{ $msg }

# Country / flag picker
settings-country-name = 国家名称：
settings-pick-country = 点击选择您的国家
flags-title = 选择国家
flags-filter = 筛选：
flags-none = 未找到国旗

# Translation editor: country & font
trans-endonym = 国家/地区名称：
trans-font = 字体：
trans-no-font = （无）
trans-browse = 浏览…
trans-google-fonts = Google 字体
trans-pick-country = 点击选择国家/地区
trans-font-outside = 该字体必须先安装在 assets/fonts 文件夹中。
trans-font-dir-missing = 未找到 assets/fonts 文件夹。

# Translation submission
trans-lang-endonym = 语言名称：
trans-author = 作者：
trans-submit = 发送…
trans-submit-hint = 生成 ZIP 包并打开预填好的电子邮件
trans-data-updated = 参考数据已更新（Languages.json / Countries.json）
trans-package-ready = 压缩包已准备就绪：
trans-package-error = 无法生成压缩包：

# ISO 639-3 requirement
trans-lang-not-iso = 仅支持具有 ISO 639-3 代码的语言进行翻译。

# FOMOD installer preview
menu-preview = 预览安装程序…
preview-title = FOMOD 安装程序预览
preview-refresh = 刷新
preview-assumptions = 文件假设
preview-details = 详细信息
preview-back = 返回
preview-next = 下一步
preview-install = 安装
preview-close = 关闭
preview-restart = 重启
preview-summary-title = 将要安装的文件
preview-empty = 没有文件将被安装。
preview-none-option = (无)
preview-invalid = 请完成必选项以继续。
preview-no-steps = 没有可见的步骤；请查看安装摘要。
preview-select-hint = 选择一个选项以查看其描述。
preview-col-source = 源
preview-col-dest = 目标
preview-col-priority = 优先级
preview-sel-exactlyone = 请选择一个选项。
preview-sel-atmostone = 请选择至多一个选项。
preview-sel-any = 选择任意数量的选项。
preview-sel-all = 安装所有选项。
preview-sel-atleastone = 至少选择一个选项。

# FOMOD validation (ModConfig 5.0 schema)
menu-validate = 验证 FOMOD
validate-report-title = FOMOD 验证
validate-ok = 未发现问题。FOMOD 符合模式规范。
xml-editor-schema-ok = 符合 ModConfig 5.0 模式规范。
xml-editor-schema-issues = 模式问题：
schema-line-col = 第 { $line } 行，第 { $col } 列：{ $msg }
schema-wrong-root = 意外的根元素 "{ $found }"（预期为 "{ $expected }"）。
schema-unknown = 在“{ $parent }”中出现意外元素“{ $element }”。
schema-missing = “{ $parent }”必须包含“{ $child }”。
schema-needs-one = “{ $parent }”必须至少包含一个“{ $child }”。
schema-too-many = “{ $child }”在“{ $parent }”中只能出现一次。
schema-missing-attr = “{ $element }”必须包含属性“{ $attr }”。
schema-bad-enum = { $element }/@{ $attr } 的值 "{ $value }" 无效（预期值：{ $allowed }）。
schema-choose-one = "{ $parent }" 必须恰好包含以下选项之一：{ $options }。

# Reordering (steps / groups / plugins)
reorder-before = 移至前面
reorder-after = 移至后面

# Country / language database explorer (Properties)
menu-properties = 属性…
prop-title = 国家/语言数据库
prop-tab-countries = 国家
prop-tab-languages = 语言
prop-filter = 筛选条件：
prop-official-langs = 官方语言
prop-spoken-langs = 使用语言
prop-endonym = 国家自称
prop-font = 字体
prop-spoken-in = 使用语言
prop-select-country = 选择一个国家以查看其详细信息。
prop-select-lang = 选择一种语言以查看其详细信息。

# Direct link to Nexus Mods (game slug)
btn-nexus = Nexus ↗
nexus-open-hint = 打开该游戏的 Nexus Mods 页面
