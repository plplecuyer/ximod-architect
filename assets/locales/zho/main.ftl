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
menu-open = 打开文件夹…
menu-open-file = 打开文件…
menu-save = 保存
menu-recent = 最近
menu-exit = 退出
menu-merge = 合并 FOMOD…
menu-export = 导出发行版存档…
# Menu - Options
menu-options = 选项
menu-settings = 设置…
menu-pre-save-script = 保存前脚本…
menu-post-save-script = 保存后脚本…
menu-translation = 翻译界面…
# Menu - Help
menu-help = 帮助
menu-check-updates = 检查更新…
menu-about = 关于

# Update check
update-checking = 正在检查更新…
update-up-to-date = XIMOD Architect 已是最新版本。
update-check-failed = 无法检查更新。请稍后再试。
update-available-status = 版本 { $version } 已可用。
update-banner-text = XIMOD Architect { $version } 已可用。
update-download = 下载：
update-skip = 跳过此版本
update-later = 稍后

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
label-plugin-name = 选项名称：
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
btn-add-plugin = 添加选项
btn-remove-plugin = 移除选项
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

# Bulk destination (assign one destination to a whole group or page at once)
label-group-dest = 整个组的目标位置
label-page-dest = 安装目标位置（整页）
btn-apply-group-dest = 应用于此组中的所有选项
btn-apply-page-dest = 应用于此页面上的所有选项
group-dest-hint = 为此组中每个选项的每个文件设置单一安装目标位置。
page-dest-hint = 为此页面上每个选项的每个文件设置单一安装目标位置（所有组）。
bulk-dest-nofiles = 暂无可更新的文件 — 请先为选项添加文件。
status-dest-applied = 已将目标位置应用于 { $num } 个文件。
preview-hidden-steps = 当前选择隐藏了 { $num } 个步骤。
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
settings-check-updates = 启动时检查更新
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
label-plugin-dependencies = 选项依赖项
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
validation-no-plugins = 步骤 { $step }，组 "{ $name }" 未配置选项

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
default-plugin-name = 选项 { $num }
pattern-label = 模式 { $num }

# Selection prompts
msg-select-group-first = 请先选择一个组。
msg-select-plugin-edit = 请选择要编辑的选项。
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

# Referenced-file verification (V2)
verify-no-root = 已跳过文件检查：未设置根文件夹
loc-header = 标题图片
loc-required = 必需文件
loc-conditional = 条件集 { $num }
loc-plugin = 步骤 { $step }，分组 { $group }，选项「{ $plugin }」
verify-missing-file = 缺少文件：{ $path }（{ $loc }）
verify-missing-folder = 缺少文件夹：{ $path }（{ $loc }）
verify-missing-image = 缺少图片：{ $path }（{ $loc }）
verify-absolute = 绝对路径（不可移植）：{ $path }（{ $loc }）
verify-outside = 路径超出根文件夹：{ $path }（{ $loc }）
verify-orphan = 孤立文件（未被任何选项引用）：{ $path }
conflict-certain = 目标路径冲突：「{ $path }」由 { $count } 个选项（{ $locs }）写入 — 它们会互相覆盖。
conflict-potential = 可能的目标路径冲突：「{ $path }」是 { $count } 个引用（{ $locs }）的目标 — 是否覆盖取决于选择/条件。

# Multi-FOMOD tabs & exit prompt (V2)
menu-close-fomod = 关闭 FOMOD
menu-close-all-fomods = 关闭所有 FOMOD
tab-untitled = （未命名）
msg-drop-not-fomod = 拖放的项目不是 FOMOD（未找到“fomod”文件夹）
exit-title = 未保存的更改
exit-unsaved = 有一个 FOMOD 尚未保存。要保存吗？
tab-close-hint = 关闭此 FOMOD
menu-new-from-folder = 从文件夹新建…
menu-templates = 模板…
templates-title = 可重用模板
templates-empty = 尚未保存任何模板。保存上方选定的步骤即可创建一个。
templates-insert = 插入
templates-save-step = 保存选定步骤
templates-name-hint = 模板名称（可选）
msg-wizard-success = 已从文件夹创建框架：{ $num } 个选项。
msg-wizard-error = 错误：{ $error }
msg-template-saved = 模板已保存：{ $name }
msg-template-inserted = 模板已插入项目。
msg-template-no-step = 请先选择一个步骤以将其保存为模板。
msg-template-no-dir = 找不到模板文件夹。
msg-drop-assigned = 已向选项添加 { $added } 个来源（根目录之外的 { $rejected } 个已忽略）。
menu-compare = 与…比较
compare-title = FOMOD 比较
compare-none = 没有差异。
btn-optimize-image = 优化图像
msg-image-optimized = 页眉图像已优化。
msg-image-ok = 页眉图像已在限制范围内。
msg-no-header-image = 没有可优化的页眉图像。
verify-image-large = 图像过大（{ $width }×{ $height }）：{ $path }
verify-image-format = 不支持的图像格式（.{ $ext }）：{ $path }
verify-image-unreadable = 无法读取的图像：{ $path }
menu-condition-editor = 条件编辑器…
condeditor-title = 条件编辑器
condeditor-set-by = 设置者：
condeditor-used-by = 使用者：
condeditor-filedeps = 文件依赖
condeditor-empty = 此项目中没有标记或依赖。
condeditor-orphan-set = 已设置但从未使用
condeditor-orphan-used = 已使用但从未设置
msg-img-optimized = 图像已优化。
msg-img-ok = 图像已在限制范围内。
msg-img-none = 没有可优化的图像。
msg-crash-recovery = 上一次会话意外结束。您的项目备份已保存到 { $path }
export-progress-title = 正在创建分发压缩包…
export-progress-files = { $done } / { $total } 个文件
msg-export-cancelled = 已取消导出；不完整的压缩包已删除。
verify-running = 正在检查磁盘上的文件…
verify-stale = 注意：检查文件时项目已更改，请重新运行验证。
prop-col-name = 名称
menu-save-as = 另存为…
menu-project = 项目
menu-tools = 工具
menu-manual = 用户手册
msg-manual-missing = 未在应用程序旁找到用户手册（PDF）。
toolbar-new = 新建
toolbar-open = 打开
toolbar-save = 保存
toolbar-validate = 验证
toolbar-preview = 预览
toolbar-export = 导出
dialog-choose-root = 选择模组的根文件夹
exit-unsaved-docs = 未保存：{ $names }
status-summary = { $steps } 个步骤 · { $options } 个选项
section-groups = 分组
section-options = 选项
section-flags = 条件标志
section-files = 要安装的文件
hint-group-type = 安装程序让用户在此分组中选择选项的方式。
hint-default-type = 没有任何依赖模式匹配时选项的提供方式：必需、可选、推荐、不可用…
hint-operator = 所有条件都必须成立（且），或任一条件成立（或）。
hint-flags = 标志是此选项被选中时设置的命名值。其他步骤和选项可以检测它们以显示、隐藏或成为必需。
hint-plugin-dependencies = 根据标志或游戏中存在的文件改变选项类型的模式：例如已安装另一个模组时为“必需”。
hint-files = 选中此选项时复制到游戏 Data 文件夹的文件和文件夹。目标路径相对于 Data；写入同一文件时优先级高者获胜。
hint-visibility = 显示此步骤所需满足的条件。留空则始终显示。
seltype-exactly-one = 恰好一个（必选）
seltype-at-most-one = 最多一个
seltype-any = 任意数量
seltype-all = 全部（无需选择）
seltype-at-least-one = 至少一个
plugtype-required = 必需
plugtype-optional = 可选
plugtype-recommended = 推荐
plugtype-not-usable = 不可用
plugtype-could-be-usable = 可能可用
plugtype-required-hint = 始终安装；用户无法取消勾选。
plugtype-optional-hint = 默认未勾选；由用户决定。
plugtype-recommended-hint = 默认勾选；用户可取消。
plugtype-not-usable-hint = 灰显且无法选择。
plugtype-could-be-usable-hint = 可选，但安装程序会警告其可能无法正常工作。
op-and = 所有条件（且）
op-or = 任一条件（或）
theme-dark = 深色
theme-light = 浅色
theme-system = 跟随系统
condeditor-setter-loc = 步骤 { "{step}" } / 分组 { "{group}" } / 「{ "{name}" }」
condeditor-pattern-of = 「{ "{name}" }」的模式 → { "{type}" }
condeditor-visibility-of = 步骤 { "{step}" } 的可见性
condeditor-cond-set = 条件集 { "{num}" }
condeditor-needs = { "{ctx}" }（需要 = { "{value}" }）
condeditor-file-dep = { "{ctx}" }：文件「{ "{name}" }」（{ "{state}" }）
menu-translate-fomod = 翻译 FOMOD…
ftr-title = 翻译 FOMOD
ftr-open-folder = 打开模组文件夹…
ftr-from-active = 从当前项目
ftr-from-active-hint = 翻译主窗口中打开的项目的 FOMOD（必须先保存）。
ftr-no-fomod = 未加载 FOMOD。
ftr-encoding = 原始文件的编码；翻译后的文件将以相同编码写入。
ftr-source-lang = 从
ftr-target-lang = 译为
ftr-lang-locked = （加载 FOMOD 后语言即固定）
ftr-translator = 译者：
ftr-save = 保存翻译
ftr-export = 导出翻译后的文件
ftr-export-sibling = 导出到 fomod_<语言> 文件夹
ftr-export-sibling-hint = 将翻译后的 info.xml 和 ModuleConfig.xml 写入原 fomod 文件夹旁边；不会改动原始文件。
ftr-export-inplace = 覆盖原始文件
ftr-export-inplace-hint = 为每个文件创建带时间戳的 .bak 副本后，替换 fomod/info.xml 和 fomod/ModuleConfig.xml。
ftr-force-explicit-order = 保持原有顺序
ftr-warn-order = 按名称排序的列表（order="Ascending"）会在模组管理器中按翻译后的名称重新排序。此选项会强制使用 order="Explicit"，使选项保持当前顺序。
ftr-update = 从文件夹更新
ftr-update-hint = 从磁盘重新读取 FOMOD 并与翻译合并：会报告新增、已更改和已删除的字符串。
ftr-preview-translated = 预览译文
ftr-progress = 已翻译 { $done } / { $total }
ftr-filter-all = 全部
ftr-filter-untranslated = 未翻译
ftr-filter-review = 待审阅
ftr-filter-issues = 有问题
ftr-filter-locked = 已锁定
ftr-type-all = 所有字段
ftr-type-names = 名称
ftr-type-descriptions = 描述
ftr-type-meta = 模组信息
ftr-search-hint = 搜索原文、译文或上下文…
ftr-next-untranslated = 下一个未翻译项
ftr-show-whitespace = 显示空格和换行符
ftr-discard-question = 当前翻译有未保存的修改。是否放弃这些修改并加载另一个 FOMOD？
ftr-discard-yes = 放弃
ftr-unsaved-close = 翻译有未保存的修改。
ftr-col-num = #
ftr-col-status = { "" }
ftr-col-context = 上下文
ftr-col-source = 原文
ftr-col-target = 译文
ftr-col-issues = { "" }
ftr-empty-hint = 打开模组文件夹或加载当前项目，以列出其可翻译的字符串。
ftr-empty-filter = 没有符合当前筛选条件的字符串。
ftr-select-row = 选择一行以编辑其译文。
ftr-copy-source = 复制原文
ftr-clear-target = 清空
ftr-lock = 不翻译
ftr-lock-hint = 已锁定的字符串将原样写入（作者、网站、专有名称…）。
ftr-note = 备注：
ftr-status-untranslated = 未翻译
ftr-status-translated = 已翻译
ftr-status-auto = 已自动预填 — 请审阅
ftr-status-fuzzy = 翻译后原文已更改 — 请审阅
ftr-status-obsolete = FOMOD 中已不存在
ftr-status-locked = 已锁定（原样写入）
ftr-field-info-name = 模组名称 (info.xml)
ftr-field-module-name = 安装程序标题 (ModuleConfig.xml)
ftr-field-author = 作者
ftr-field-website = 网站
ftr-field-description = 模组描述
ftr-field-step = 步骤名称
ftr-field-group = 组名称
ftr-field-plugin = 选项名称
ftr-field-plugin-desc = 选项描述
ftr-issue-empty = 译文为空
ftr-issue-whitespace = 译文只包含空格
ftr-issue-edge-whitespace = 开头或结尾的空格与原文不同
ftr-issue-token = 受保护的标记不一致 — 缺少：{ $missing } ；多余：{ $extra }
ftr-issue-newline-name = 名称不能包含换行符
ftr-issue-control = 包含 XML 无法存储的字符
ftr-issue-length = 与原文相比长度异常 (×{ $ratio })
ftr-issue-identical = 与原文相同
ftr-issue-duplicate = 相同的原文在 { $key } 中译法不同
ftr-issue-cdata = 此处不允许出现 ]]> 序列
ftr-load-error = 无法加载 FOMOD：{ $error }
ftr-extracted = 找到 { $num } 个可翻译的字符串。
ftr-sidecar-found = 已加载并合并现有翻译：新增 { $new }，更改 { $changed }，删除 { $removed }。
ftr-saved = 翻译已保存到 { $path }
ftr-save-error = 无法保存翻译：{ $error }
ftr-save-first = 请先保存项目，然后再翻译。
ftr-export-success = 已将 { $count } 个字符串写入 { $path }
ftr-export-error = 导出失败：{ $error }
ftr-export-blocked = 导出前必须修复 { $num } 个阻碍性问题。
ftr-export-stale = 由于 FOMOD 已更改，跳过了 { $num } 个字符串；请使用「从文件夹更新」。
ftr-update-report = 已更新：新增 { $new }，更改 { $changed }，移动 { $moved }，删除 { $removed }，未变 { $unchanged }。
menu-edit = 编辑
menu-undo = 撤销
menu-redo = 重做
tree-title = 项目
tree-mod-info = 模组信息
tree-steps = 安装步骤
tree-required = 必需文件
tree-conditional = 条件安装项
tree-empty-steps = 尚无步骤 — 点击 + 添加一个。
tree-duplicate = 复制
tree-delete = 删除
tree-save-template = 另存为模板…
tree-drop-hint = 拖放到此处以移动
cond-set-label = 条件集 { $num }
inspector-empty = 请在项目树中选择一项，或添加一个步骤以开始。
count-options = { $num } 个选项
count-files = { $num } 个文件
msg-deleted-undo = 已删除。使用“撤销”(Ctrl+Z) 可将其恢复。
problems-title = 问题
problems-errors = { $num } 个错误
problems-warnings = { $num } 个警告
btn-close = 关闭
ftr-export-package = 作为翻译包（存档）
ftr-export-package-hint = 生成可直接上传的 .zip 或 .7z：翻译后的 info.xml 和 ModuleConfig.xml 以及一份 README（仅补丁），或包含翻译后文件的整个模组（完整）。
ftr-package-full = 完整模组
ftr-package-full-hint = 将模组的所有文件都放入存档，而不只是两个翻译后的 XML 文件。请确认作者允许再分发。
ftr-package-name-template = 名称：
ftr-readme-patch = 此存档包含「{ $name }」安装程序的翻译（语言：{ $langname }；fomod/info.xml 和 fomod/ModuleConfig.xml）。请将其安装并覆盖原模组，或让模组管理器合并，使翻译后的文件替换原始文件。仅安装程序的文本有变化；不包含模组本身的文件。由 XIMOD Architect 制作。
ftr-readme-full = 此存档包含「{ $name }」，其安装程序已翻译（语言：{ $langname }；fomod/info.xml 和 fomod/ModuleConfig.xml）。请按原模组的方式安装。仅安装程序的文本有改动。由 XIMOD Architect 制作。
ftr-apply-memory = 从记忆库填充
ftr-memory-size = 翻译记忆库：此语言对共有 { $num } 个条目。每条保存的翻译都会加入其中。
ftr-memory-applied = 已从翻译记忆库填充 { $num } 个字符串（标记为「待审阅」）。
ftr-memory-suggestion = 记忆库建议：
ftr-use-suggestion = 使用
ftr-propagate = 应用到相同原文
ftr-propagate-hint = 将此翻译复制到原文相同且尚未翻译的所有其他字符串。
ftr-propagated = 已填充 { $num } 个相同的字符串。
ftr-csv-export = 导出 CSV…
ftr-csv-import = 导入 CSV…
ftr-csv-imported = 已从 CSV 文件更新 { $num } 个字符串。
ftr-csv-error = CSV 错误：{ $error }
ftr-glossary = 术语表
ftr-glossary-source = 术语
ftr-glossary-target = 译文
ftr-glossary-case = 大小写
ftr-glossary-dnt = 保留
ftr-glossary-add = 添加术语
ftr-issue-glossary = 术语表：「{ $term }」未按预期翻译

# ---- Lot F1: open from archive, import fidelity ----
menu-open-archive = 打开压缩包…
filter-archive = 模组压缩包 (zip, 7z)
msg-archive-opened = 压缩包已打开（已解压 { $num } 个文件）：{ $path }
msg-archive-reused = 压缩包已解压，重新使用 { $path }
msg-archive-unsupported = 不支持压缩包格式「.{ $ext }」；请先用 7-Zip 解压（只能打开 .zip 和 .7z）。
msg-archive-error = 打开压缩包时出错：{ $error }
msg-archive-no-fomod = 压缩包中未找到「fomod」文件夹（{ $path }）
msg-archive-extracting = 正在解压压缩包…
ftr-open-archive = 打开模组压缩包…
ftr-package-full-partial = 该模组是从仅包含 fomod 文件夹的压缩包打开的；完整包需要已解压的模组。
info-module-deps = 模组要求
info-module-deps-hint = 安装程序运行前整个模组所需的文件或标志 (moduleDependencies)。如无则留空。
info-header-advanced = 高级页眉
info-title-position = 标题位置
info-title-colour = 标题颜色
info-title-colour-hint = 应为六位十六进制数字 (RRGGBB)
info-image-show = 显示页眉图片
info-image-fade = 淡化页眉图片
info-image-height = 页眉图片高度
info-attr-default = (默认)
file-always-install = 始终
file-always-install-hint = 即使未选中该选项，也始终安装此文件 (alwaysInstall)。
file-install-if-usable = 可用时
file-install-if-usable-hint = 只要该选项可用就安装此文件，即使未选中 (installIfUsable)。
msg-import-lossy = 此 FOMOD 包含 { $num } 个 XIMOD 无法编辑的结构；保存项目时将被丢弃。
fidelity-nested-deps = { $context } 中存在嵌套的依赖组（仅支持一级）
fidelity-game-dep = { $context } 中存在游戏版本要求 { $version }
fidelity-fomm-dep = { $context } 中存在模组管理器版本要求 { $version }
fidelity-unknown = 「{ $parent }」中的元素「{ $element }」不受支持（{ $context }）
loc-module = 模组要求
loc-step = 步骤 { $step }「{ $name }」
loc-installer = 安装程序

# ---- Lot F2: rotating backups, plugin masters, light-plugin checks ----
menu-restore-backup = 恢复备份…
backups-title = 恢复备份
backups-empty = 此项目尚无备份。每次在先前版本上保存项目时都会创建一个备份。
backups-changes = 与当前项目有 { $num } 处差异
btn-compare = 比较
btn-restore = 恢复
btn-delete-backups = 删除所有备份
btn-delete-backups-confirm = 再次点击以删除所有备份
msg-backup-restored = 已将 { $time } 的备份恢复到编辑器中（尚未保存；「撤销」可还原此操作）
msg-backups-deleted = 已删除 { $num } 个备份
settings-backup-count = 保留的备份数：
settings-backup-count-hint = 保存时在 fomod/backups 下保留的 FOMOD XML 先前版本数量（0 = 不备份）。
settings-autosave-minutes = 自动保存恢复副本的间隔（分钟）：
settings-autosave-minutes-hint = 每隔此时间，所有已修改项目的恢复副本都会写入配置文件夹；仅在异常退出后，下次启动时才会提示（0 = 关闭）。
settings-auto-masters = 将插件的主文件添加为条件
settings-auto-masters-hint = 将插件（.esp/.esm/.esl）添加到选项时，其所需、且游戏和本模组均未提供的主文件将成为该选项的「Active」文件条件。
msg-author-from-plugin = 已从插件头部填入作者：{ $author }
msg-masters-added = 已将 { $plugin } 的 { $num } 个主文件添加为文件条件
issue-missing-master = { $plugin } 需要 { $master }，但它既不在本模组中，也未声明为依赖项
issue-esl-mismatch-flag = { $plugin } 的扩展名为 .esl，但未设置其 light (ESL) 标志
issue-esl-eligible = { $plugin } 可以标记为 light（{ $num } 条新记录，上限 { $limit }）
issue-esl-too-big = { $plugin } 已标记为 light，但不符合 light 插件规则（{ $num } 条新记录，上限 { $limit }，或存在超出允许范围的 FormID）
menu-plugin-report = 插件报告…
plugins-title = 插件报告
plugins-file = 文件
plugins-kind = 类型
plugins-light = Light 标志
plugins-masters = 主文件
plugins-new-records = 新记录 / 上限
plugins-eligible = 可设为 light
plugins-empty = 此项目不安装任何插件文件（.esp、.esm 或 .esl）。
plugins-unreadable = 无法读取

# --- V2 lot F3: richer simulator, install sizes, Nexus description ---
preview-tree = 最终文件树
preview-total-size = 安装总大小：{ $size }
preview-tree-truncated = 文件树已截断：文件过多，无法全部展开（上方的大小仅为部分统计）。
preview-overwritten-by = 被 { $plugin } 覆盖
preview-scenario = 场景：
preview-scenario-load = 加载
preview-scenario-save = 保存…
preview-scenario-delete = 删除
preview-scenario-name = 场景名称
preview-scenario-saved = 场景“{ $name }”已保存到 fomod/scenarios
preview-scenario-unresolved = 场景中有 { $num } 项选择与本项目的任何选项都不匹配（已重命名或已移除）
preview-scenario-none = (无场景)
issue-unreachable-step = 步骤“{ $step }”永远不会显示：其可见性条件检测的标志值没有任何先前的选项设置
issue-unreachable-option = 选项“{ $plugin }”永远无法被选中：其可用类型模式检测的标志值没有任何选项设置
issue-unreachable-cond = 条件文件集 { $num } 永远不会生效：其条件检测的标志值没有任何选项设置
size-option = 安装大小：{ $size }（{ $num } 个文件）
size-missing = { $num } 个源文件缺失
size-unknown = 安装大小：—（运行“验证”以测量）
menu-nexus-desc = Nexus 描述…
nexus-title = Nexus Mods 描述
nexus-format = 格式：
nexus-include-requirements = 前置要求
nexus-include-options = 安装选项
nexus-include-install = 安装
nexus-include-changelog = 更新日志
nexus-previous = 上一版本…
nexus-previous-none = (无上一版本：不生成更新日志)
nexus-language = 语言：
nexus-language-source = (源语言)
nexus-sec-requirements = 前置要求
nexus-sec-options = 安装选项
nexus-sec-install = 安装
nexus-sec-changelog = 更新日志
nexus-install-text = 本模组附带 FOMOD 安装程序：请使用模组管理器（Vortex、Mod Organizer 2）安装，并在安装程序中选择所需选项。
nexus-requires = 需要
nexus-step = 步骤
nexus-added = 新增
nexus-removed = 移除
nexus-changed = 更改
btn-copy = 复制
btn-save-as = 另存为…
msg-copied = 已复制到剪贴板
msg-saved-to = 已保存到 { $path }

# --- V2 lot G1: editing condition editor, project strings ---
condeditor-rename = 重命名…
condeditor-rename-exists = 已存在名为“{ $name }”的标志
condeditor-renamed = 标志“{ $from }”已重命名为“{ $to }”（{ $num } 处）
condeditor-delete-uses = 删除所有使用
condeditor-deleted-uses = 标志“{ $name }”已从所有位置移除（{ $num } 处）
condeditor-values-set = 设置的值：
condeditor-values-tested = 检测的值：
condeditor-value-never-set = { $value } — 被检测但从未设置
condeditor-value-never-tested = { $value } — 被设置但从未检测
condeditor-builder = 条件构建器
condeditor-builder-none = 在主窗口中选择步骤、选项、条件文件集或模组信息，即可在此编辑其条件。
condeditor-builder-pattern = 模式：
condeditor-sentence-if = 如果
condeditor-sentence-and = 且
condeditor-sentence-or = 或
condeditor-sentence-flag = 标志 { "{name}" } = { "{value}" }
condeditor-sentence-file = 文件 { "{name}" } 为 { "{value}" }
condeditor-sentence-empty = (无条件：始终为真)
condeditor-sentence-then-visible = 则 显示该步骤
condeditor-sentence-then-type = 则 该选项变为 { $type }
condeditor-sentence-then-install = 则 安装这些文件
condeditor-sentence-then-module = 则 安装程序可以运行（在启动前检查）
issue-flag-value-never-set = 标志“{ $flag }”以值“{ $value }”进行检测，但没有任何选项设置该值
issue-flag-never-used = 标志“{ $flag }”已设置，但从未在任何地方检测
menu-project-strings = 项目字符串…
strings-title = 项目字符串
strings-search = 搜索文本、位置或键…
strings-kind-all = 全部
strings-kind-names = 名称
strings-kind-descriptions = 描述
strings-duplicates-only = 仅重复项
strings-replace-with = 替换为：
strings-case = 区分大小写
strings-whole-word = 全字匹配
strings-replace-current = 替换
strings-replace-all = 全部替换
strings-replaced = 已替换 { $num } 个字符串
strings-dup-badge = ×{ $num }
strings-dup-hover = 与以下项文本相同：
strings-count = { $num } 个字符串 · { $dups } 个重复组
strings-col-location = 位置
strings-col-field = 字段
strings-col-text = 文本

# Lot G2 — Bethesda archive readers (BSA/BA2)
menu-archive-contents = 压缩包内容…
filter-bethesda-archive = Bethesda 压缩包 (bsa, ba2)
archive-view-title = 压缩包内容
archive-view-format = 格式：
archive-view-entries = { $num } 个条目
archive-view-size = 解压后 { $size }
archive-view-search = 搜索路径…
archive-view-col-path = 路径
archive-view-col-size = 大小
archive-view-col-compressed = 已压缩
archive-view-truncated = 仅显示前 { $num } 个匹配条目 — 请缩小搜索范围。
archive-view-error = 无法读取此压缩包：{ $error }
archive-view-hint = 查看此压缩包的内容
issue-conflict-archive = 多个压缩包中存在相同资源：「{ $path }」由 { $count } 个引用（{ $locs }）打包 — 游戏的压缩包加载顺序决定使用哪一个。
issue-conflict-archive-loose = 压缩包与散装文件：「{ $path }」既打包在压缩包中，又作为散装文件安装（{ $locs }）— 散装文件优先于压缩包中的文件。
preview-in-archive = (压缩包内)
preview-archived-size = 其中 { $size } 打包在压缩包中

# --- Project tree: expand / collapse menus
tree-expand = 展开
tree-collapse = 折叠
tree-expand-all = 全部展开
tree-expand-selected = 展开所选项
tree-expand-from = 从所选项展开
tree-collapse-all = 全部折叠
tree-collapse-selected = 折叠所选项
tree-collapse-from = 从所选项折叠
tree-expand-all-hint = 展开所有标题
tree-expand-selected-hint = 仅展开所选标题
tree-expand-from-hint = 展开所选标题及其下的所有内容
tree-collapse-all-hint = 折叠所有标题
tree-collapse-selected-hint = 仅折叠所选标题
tree-collapse-from-hint = 折叠所选标题及其下的所有内容

# --- Lot N: nested dependency groups, version conditions
btn-add-group-cond = 添加组
btn-remove-group-cond = 移除组
dep-type-game = 游戏版本
dep-type-fomm = 模组管理器版本
dep-group-hint = 以“且”/“或”组合的一组条件；组可以嵌套。
condeditor-sentence-game = 游戏版本 ≥ { "{value}" }
condeditor-sentence-fomm = 模组管理器版本 ≥ { "{value}" }

# --- FOMOD translator: unique texts mode
ftr-uniques = 唯一文本
ftr-uniques-hint = 每个不同的原文只显示一行。翻译该行即可一次性翻译所有原文相同的字符串。
ftr-uniques-synced = 已更新 { $num } 个相同的字符串。
ftr-uniques-group = { $num } 个字符串共用此文本；其翻译适用于全部。
