# XIMOD Architect - translation metadata
# @language = kor
# @font = Noto_Sans_KR/static/NotoSansKR-Regular.ttf
# @langname = 한국어
# @author = XIMOD Team

# XIMOD Architect - English Translations

# Application
app-title = XIMOD Architect
app-version = 버전 { $version }

# Status messages
status-ready = 준비 완료
msg-save-success = FOMOD가 성공적으로 저장되었습니다
msg-save-error = FOMOD 저장 중 오류가 발생했습니다
msg-export-success = 배포 아카이브 생성됨 ({ $count }개 파일): { $path }
msg-export-error = 배포 아카이브 생성 중 오류가 발생했습니다: { $error }
msg-load-success = FOMOD가 성공적으로 로드되었습니다
msg-load-error = FOMOD 불러오기 오류
msg-merge-success = FOMOD 병합 성공
msg-merge-error = FOMOD 병합 오류
msg-no-root-selected = 먼저 루트 디렉터리를 선택해 주세요
msg-no-fomod-folder = 'fomod' 폴더를 찾을 수 없습니다. 생성하시겠습니까?
msg-file-outside-root = 파일이 루트 디렉터리 밖에 있습니다

# Menu - File
menu-file = 파일
menu-new = 새로 만들기
menu-open = 폴더 열기…
menu-open-file = 파일 열기…
menu-save = 저장
menu-recent = 최근 항목
menu-exit = 종료
menu-merge = FOMOD 병합…
menu-export = 배포 아카이브 내보내기…
# Menu - Options
menu-options = 옵션
menu-settings = 설정…
menu-pre-save-script = 저장 전 스크립트…
menu-post-save-script = 저장 후 스크립트…
menu-translation = 인터페이스 번역…
# Menu - Help
menu-help = 도움말
menu-check-updates = 업데이트 확인…
menu-about = 정보

# Update check
update-checking = 업데이트 확인 중…
update-up-to-date = XIMOD Architect는 최신 버전입니다.
update-check-failed = 업데이트를 확인할 수 없습니다. 나중에 다시 시도하세요.
update-available-status = 버전 { $version }을(를) 사용할 수 있습니다.
update-banner-text = XIMOD Architect { $version }을(를) 사용할 수 있습니다.
update-download = 다운로드:
update-skip = 이 버전 건너뛰기
update-later = 나중에

# Tabs
tab-info = 모드 정보
tab-steps = 설치 단계
tab-required = 필수 설치 항목
tab-conditional = 조건부 설치 항목

# Info Tab
label-workspace = 작업 공간
label-root-dir = 루트 디렉터리:
label-mod-name = 모드 이름:
label-author = 제작자:
label-version = 버전:
label-game-name = 게임 이름:
label-category = 카테고리:
label-url = 웹사이트 URL:
label-header-image = 헤더 이미지:
label-description = 설명:
placeholder-select-dir = (디렉터리 선택)
placeholder-select-game = (게임 선택)

# Steps Tab
label-step-name = 단계 이름:
label-group-name = 그룹 이름:
label-group-type = 그룹 유형:
label-plugin-name = 옵션 이름:
label-plugin-desc = 설명:
label-plugin-type = 기본 유형:
label-plugin-image = 이미지:
label-visibility = 표시 조건
label-operator = 연산자:

# Buttons
btn-browse = 찾아보기...
btn-clear = 지우기
btn-add = 추가
btn-remove = 제거
btn-add-step = 새 단계
btn-delete-step = 단계 삭제
btn-add-group = 그룹 추가
btn-remove-group = 그룹 제거
btn-add-plugin = 옵션 추가
btn-remove-plugin = 옵션 제거
btn-add-file = 파일 추가
btn-add-folder = 폴더 추가
btn-remove-file = 제거
btn-add-flag = 플래그 추가
btn-remove-flag = 플래그 제거
btn-add-condition = 조건 추가
btn-remove-condition = 조건 제거
btn-add-dependency = 종속성 추가
btn-remove-dependency = 종속성 제거
btn-add-pattern = 새 패턴
btn-remove-pattern = 패턴 삭제
btn-save = 저장
btn-cancel = 취소
btn-ok = 확인
btn-yes = 예
btn-no = 아니요

# Condition/Dependency Labels
label-flag-name = 플래그 이름:
label-flag-value = 값:
label-condition-type = 유형:
label-condition-name = 이름:
label-condition-value = 값:
label-dep-type = 의존성 유형:
label-dep-name = 이름/파일:
label-dep-value = 값/상태:

# Files
label-source = 소스
label-destination = 대상
label-priority = 우선순위
label-file-type = 유형

# Bulk destination (assign one destination to a whole group or page at once)
label-group-dest = 전체 그룹 대상 위치
label-page-dest = 설치 대상 위치 (전체 페이지)
btn-apply-group-dest = 이 그룹의 모든 옵션에 적용
btn-apply-page-dest = 이 페이지의 모든 옵션에 적용
group-dest-hint = 이 그룹의 각 옵션의 모든 파일에 단일 설치 대상을 설정합니다.
page-dest-hint = 이 페이지의 각 옵션의 모든 파일에 단일 설치 대상을 설정합니다 (모든 그룹).
bulk-dest-nofiles = 아직 업데이트할 파일이 없습니다 — 먼저 옵션에 파일을 추가하세요.
status-dest-applied = { $num }개 파일에 대상을 적용했습니다.
preview-hidden-steps = 현재 선택으로 { $num }개 단계가 숨겨졌습니다.
label-files = 파일
label-dependencies = 종속성

# Settings Dialog
settings-title = 설정
settings-tab-general = 일반
settings-tab-recent-files = 최근 파일
settings-language = 언어:
settings-theme = 테마:
settings-font-size = 글꼴 크기:
settings-replace-newlines = 설명 내의 줄바꿈 처리
settings-check-updates = 시작 시 업데이트 확인
settings-max-recent = 최근 파일 최대 개수:
settings-window-width = 창 너비:
settings-window-height = 창 높이:
settings-no-recent-files = 최근 파일이 없습니다.

# Status messages for settings
status-settings-saved = 설정이 성공적으로 저장되었습니다

# About Dialog
about-title = XIMOD Architect 정보
about-description = 베데스다 게임 모드용 크로스 플랫폼 FOMOD 설치 프로그램 생성 도구입니다.
about-license = MIT 라이선스 하에 배포됩니다
about-copyright = © 2025-2026 XIMOD Team
about-credit = Wenderer의 원본 도구를 Rust로 포팅한 버전:

# Script Dialog
script-title = 스크립트 편집
script-info = 스크립트는 저장 전이나 후에 실행됩니다. 다음 매크로를 사용할 수 있습니다:
script-macros = 사용 가능한 매크로:
macro-modname = $MODNAME$ - 모드 이름
macro-modauthor = $MODAUTHOR$ - 제작자 이름
macro-modversion = $MODVERSION$ - 모드 버전
macro-modroot = $MODROOT$ - 루트 디렉터리 경로
macro-date = $DATE$ - 현재 날짜 (YYYY-MM-DD)
macro-time = $TIME$ - 현재 시간 (HH:MM:SS)
macro-random = $RANDOM$ - 난수

# Plugin Dependencies
label-plugin-dependencies = 옵션 종속성
label-default-type = 기본 유형:
label-pattern-type = 패턴 유형:
label-pattern-operator = 패턴 연산자:

# Conditional Files
label-pattern = 패턴

# Validation Messages
validation-no-name = 모드 이름이 필수입니다
validation-no-steps = 단계 또는 필수 파일이 하나 이상 필요합니다
validation-empty-step = 단계 { $num }에 이름이 없습니다
validation-empty-group = 단계 { $step }, 그룹 { $group }에 이름이 없습니다
validation-no-plugins = 단계 { $step }, 그룹 "{ $name }"에 옵션이 없습니다

# File States
state-active = 활성
state-inactive = 비활성
state-missing = 누락됨

# Confirmation
confirm-title = 확인
confirm-delete = 이 항목을 정말로 삭제하시겠습니까?
confirm-discard = 저장되지 않은 변경 사항이 있습니다. 변경 사항을 취소하고 계속하시겠습니까?
confirm-unsaved = 저장되지 않은 변경 사항이 있습니다. 닫기 전에 저장하시겠습니까?
confirm-save-issues = 프로젝트에 다음과 같은 문제가 있습니다:
confirm-save-anyway = 그래도 저장하시겠습니까?

# Errors
error-invalid-xml = 유효하지 않은 XML 파일
error-parse-failed = FOMOD 구문 분석에 실패했습니다
error-write-failed = 파일 쓰기에 실패했습니다
error-create-dir = 디렉터리 생성에 실패했습니다

# Default names (generated when creating new items)
default-step-name = 단계 { $num }
default-group-name = 그룹 { $num }
default-plugin-name = 옵션 { $num }
pattern-label = 패턴 { $num }

# Selection prompts
msg-select-group-first = 먼저 그룹을 선택하십시오.
msg-select-plugin-edit = 편집할 옵션을 선택하십시오.
label-empty = (비어 있음)
image-no-image = 이미지가 없습니다

# File dialog filters
filter-images = 이미지
filter-xml = XML

# Dependency types
dep-type-flag = 플래그
dep-type-file = 파일

# Status bar
status-modified = 수정됨

# Status messages (errors)
msg-settings-save-error = 설정 저장 오류
msg-script-save-error = 스크립트 저장 오류

# Translation editor
trans-title = 번역 편집기
trans-source-lang = 표시된 언어:
trans-target-lang = 번역할 언어:
trans-col-key = 키
trans-col-source = 레이블
trans-col-target = 번역
trans-saved = 번역이 저장되었습니다
trans-save-error = 번역 저장 오류

# XML editor
xml-editor-title = XML 편집기
xml-editor-edit = 편집
xml-editor-apply = 적용
xml-editor-revert = 취소
xml-editor-readonly = 읽기 전용
xml-editor-editing = 편집 중 — 그래픽 탭이 잠겨 있습니다
xml-editor-error = 오류:
xml-editor-applied = XML 변경 사항이 적용되었습니다
xml-editor-wellformed = 구문 구조가 올바른 XML
xml-editor-error-at = { $line }행, { $col }열: { $msg }

# Country / flag picker
settings-country-name = 국가 이름:
settings-pick-country = 클릭하여 국가를 선택하세요
flags-title = 국가 선택
flags-filter = 필터:
flags-none = 국기를 찾을 수 없음

# Translation editor: country & font
trans-endonym = 국가 명칭:
trans-font = 글꼴:
trans-no-font = (없음)
trans-browse = 찾아보기…
trans-google-fonts = Google Fonts
trans-pick-country = 클릭하여 국가를 선택하세요
trans-font-outside = 글꼴은 먼저 assets/fonts 폴더에 설치되어야 합니다.
trans-font-dir-missing = assets/fonts 폴더를 찾을 수 없습니다.

# Translation submission
trans-lang-endonym = 언어 명칭:
trans-author = 작성자:
trans-submit = 보내기…
trans-submit-hint = zip 파일을 생성하고 미리 작성된 이메일을 열어보세요
trans-data-updated = 참조 데이터가 업데이트되었습니다 (Languages.json / Countries.json)
trans-package-ready = 아카이브 준비 완료:
trans-package-error = 아카이브를 생성할 수 없습니다:

# ISO 639-3 requirement
trans-lang-not-iso = ISO 639-3 코드가 있는 언어에 대해서만 번역이 가능합니다.

# FOMOD installer preview
menu-preview = 설치 프로그램 미리 보기…
preview-title = FOMOD 설치 프로그램 미리 보기
preview-refresh = 새로 고침
preview-assumptions = 파일 가정
preview-details = 세부 정보
preview-back = 뒤로
preview-next = 다음
preview-install = 설치
preview-close = 닫기
preview-restart = 다시 시작
preview-summary-title = 설치될 파일
preview-empty = 설치될 파일이 없습니다.
preview-none-option = (없음)
preview-invalid = 계속하려면 필수 항목을 선택하십시오.
preview-no-steps = 표시된 단계가 없습니다. 설치 요약을 참조하십시오.
preview-select-hint = 옵션을 선택하면 해당 설명을 볼 수 있습니다.
preview-col-source = 소스
preview-col-dest = 대상
preview-col-priority = 우선순위
preview-sel-exactlyone = 정확히 하나의 옵션만 선택하십시오.
preview-sel-atmostone = 최대 하나의 옵션만 선택하십시오.
preview-sel-any = 원하는 수의 옵션을 선택하십시오.
preview-sel-all = 모든 옵션이 설치됩니다.
preview-sel-atleastone = 최소 한 가지 옵션을 선택하십시오.

# FOMOD validation (ModConfig 5.0 schema)
menu-validate = FOMOD 유효성 검사
validate-report-title = FOMOD 유효성 검사
validate-ok = 문제가 발견되지 않았습니다. FOMOD가 스키마를 준수합니다.
xml-editor-schema-ok = ModConfig 5.0 스키마를 준수합니다.
xml-editor-schema-issues = 스키마 문제:
schema-line-col = { $line } 행, { $col } 열: { $msg }
schema-wrong-root = 예상치 못한 루트 "{ $found }" (예상: "{ $expected }").
schema-unknown = "{ $parent }" 내에 예상치 못한 요소 "{ $element }"가 있습니다.
schema-missing = "{ $parent }"에는 "{ $child }"가 포함되어야 합니다.
schema-needs-one = "{ $parent }"에는 적어도 하나의 "{ $child }"가 포함되어야 합니다.
schema-too-many = "{ $parent }" 내에서는 "{ $child }"가 한 번만 나타날 수 있습니다.
schema-missing-attr = "{ $element }"에는 속성 "{ $attr }"이 필수입니다.
schema-bad-enum = { $element }/@{ $attr }에 대한 값 "{ $value }"가 유효하지 않습니다(예상 값: { $allowed }).
schema-choose-one = "{ $parent }"에는 { $options } 중 정확히 하나만 포함되어야 합니다.

# Reordering (steps / groups / plugins)
reorder-before = 앞으로 이동
reorder-after = 뒤로 이동

# Country / language database explorer (Properties)
menu-properties = 속성…
prop-title = 국가/언어 데이터베이스
prop-tab-countries = 국가
prop-tab-languages = 언어
prop-filter = 필터:
prop-official-langs = 공식 언어
prop-spoken-langs = 사용 언어
prop-endonym = 국가 명칭
prop-font = 글꼴
prop-spoken-in = 사용 지역
prop-select-country = 국가를 선택하여 세부 정보를 확인하세요.
prop-select-lang = 언어를 선택하여 세부 정보를 확인하세요.

# Direct link to Nexus Mods (game slug)
btn-nexus = Nexus ↗
nexus-open-hint = 게임의 Nexus Mods 페이지 열기

# Referenced-file verification (V2)
verify-no-root = 파일 검증 건너뜀: 루트 폴더가 설정되지 않았습니다
loc-header = 헤더 이미지
loc-required = 필수 파일
loc-conditional = 조건부 세트 { $num }
loc-plugin = { $step }단계, { $group }그룹, 옵션 "{ $plugin }"
verify-missing-file = 누락된 파일: { $path } ({ $loc })
verify-missing-folder = 누락된 폴더: { $path } ({ $loc })
verify-missing-image = 누락된 이미지: { $path } ({ $loc })
verify-absolute = 절대 경로(이식 불가): { $path } ({ $loc })
verify-outside = 경로가 루트 폴더를 벗어남: { $path } ({ $loc })
verify-orphan = 고아 파일(어떤 옵션도 참조하지 않음): { $path }
conflict-certain = 대상 경로 충돌: “{ $path }”에 { $count }개 옵션({ $locs })이 기록됩니다 — 서로 덮어씁니다.
conflict-potential = 대상 경로 충돌 가능성: “{ $path }”은(는) { $count }개 참조({ $locs })의 대상입니다 — 덮어쓰기는 선택/조건에 따라 달라집니다.

# Multi-FOMOD tabs & exit prompt (V2)
menu-close-fomod = FOMOD 닫기
menu-close-all-fomods = 모든 FOMOD 닫기
tab-untitled = (제목 없음)
msg-drop-not-fomod = 드롭한 항목은 FOMOD가 아닙니다("fomod" 폴더 없음)
exit-title = 저장되지 않은 변경 사항
exit-unsaved = 저장되지 않은 FOMOD가 있습니다. 저장하시겠습니까?
tab-close-hint = 이 FOMOD 닫기
menu-new-from-folder = 폴더에서 새로 만들기…
menu-templates = 템플릿…
templates-title = 재사용 가능한 템플릿
templates-empty = 저장된 템플릿이 아직 없습니다. 위에서 선택한 단계를 저장하여 만드세요.
templates-insert = 삽입
templates-save-step = 선택한 단계 저장
templates-name-hint = 템플릿 이름(선택 사항)
msg-wizard-success = 폴더에서 골격을 만들었습니다: { $num }개 옵션.
msg-wizard-error = 오류: { $error }
msg-template-saved = 템플릿 저장됨: { $name }
msg-template-inserted = 템플릿이 프로젝트에 삽입되었습니다.
msg-template-no-step = 먼저 단계를 선택하여 템플릿으로 저장하세요.
msg-template-no-dir = 템플릿 폴더를 찾을 수 없습니다.
msg-drop-assigned = 옵션에 { $added }개 소스를 추가했습니다(루트 밖 { $rejected }개 무시).
menu-compare = 비교…
compare-title = FOMOD 비교
compare-none = 차이 없음.
btn-optimize-image = 이미지 최적화
msg-image-optimized = 헤더 이미지가 최적화되었습니다.
msg-image-ok = 헤더 이미지가 이미 한도 내에 있습니다.
msg-no-header-image = 최적화할 헤더 이미지가 없습니다.
verify-image-large = 이미지가 너무 큽니다({ $width }×{ $height }): { $path }
verify-image-format = 지원되지 않는 이미지 형식(.{ $ext }): { $path }
verify-image-unreadable = 읽을 수 없는 이미지: { $path }
menu-condition-editor = 조건 편집기…
condeditor-title = 조건 편집기
condeditor-set-by = 설정 위치:
condeditor-used-by = 사용 위치:
condeditor-filedeps = 파일 종속성
condeditor-empty = 이 프로젝트에는 플래그나 종속성이 없습니다.
condeditor-orphan-set = 설정되었지만 사용되지 않음
condeditor-orphan-used = 사용되지만 설정되지 않음
msg-img-optimized = 이미지가 최적화되었습니다.
msg-img-ok = 이미지가 이미 한도 내에 있습니다.
msg-img-none = 최적화할 이미지가 없습니다.
msg-crash-recovery = 이전 세션이 예기치 않게 종료되었습니다. 프로젝트 백업이 { $path }에 저장되었습니다
export-progress-title = 배포 아카이브를 만드는 중…
export-progress-files = { $done } / { $total } 파일
msg-export-cancelled = 내보내기가 취소되었습니다. 불완전한 아카이브는 삭제되었습니다.
verify-running = 디스크의 파일을 확인하는 중…
verify-stale = 참고: 파일을 확인하는 동안 프로젝트가 변경되었습니다. 검증을 다시 실행하세요.
prop-col-name = 이름
menu-save-as = 다른 이름으로 저장…
menu-project = 프로젝트
menu-tools = 도구
menu-manual = 사용자 설명서
msg-manual-missing = 사용자 설명서(PDF)를 애플리케이션 옆에서 찾을 수 없습니다.
toolbar-new = 새로 만들기
toolbar-open = 열기
toolbar-save = 저장
toolbar-validate = 검증
toolbar-preview = 미리보기
toolbar-export = 내보내기
dialog-choose-root = 모드의 루트 폴더 선택
exit-unsaved-docs = 저장되지 않음: { $names }
status-summary = { $steps }단계 · { $options }개 옵션
section-groups = 그룹
section-options = 옵션
section-flags = 조건 플래그
section-files = 설치할 파일
hint-group-type = 설치 프로그램이 이 그룹에서 사용자에게 옵션을 선택하게 하는 방식입니다.
hint-default-type = 의존성 패턴이 하나도 일치하지 않을 때 옵션이 제공되는 방식: 필수, 선택, 권장, 사용 불가…
hint-operator = 모든 조건이 참이어야 함(AND) 또는 그중 하나만(OR).
hint-flags = 플래그는 이 옵션이 선택될 때 설정하는 이름 있는 값입니다. 다른 단계와 옵션이 이를 검사하여 표시, 숨김 또는 필수 여부를 결정할 수 있습니다.
hint-plugin-dependencies = 플래그나 게임에 있는 파일에 따라 옵션 유형을 바꾸는 패턴입니다. 예: 다른 모드가 설치되어 있으면 “필수”.
hint-files = 이 옵션이 선택될 때 게임의 Data 폴더로 복사되는 파일과 폴더입니다. 대상은 Data 기준 상대 경로이며, 충돌 시 높은 우선순위가 이깁니다.
hint-visibility = 이 단계가 표시되기 위해 충족해야 하는 조건입니다. 항상 표시하려면 비워 두세요.
seltype-exactly-one = 정확히 하나(필수)
seltype-at-most-one = 최대 하나
seltype-any = 개수 제한 없음
seltype-all = 모두(선택 불가)
seltype-at-least-one = 최소 하나
plugtype-required = 필수
plugtype-optional = 선택
plugtype-recommended = 권장
plugtype-not-usable = 사용 불가
plugtype-could-be-usable = 사용 가능할 수 있음
plugtype-required-hint = 항상 설치되며 사용자가 선택을 해제할 수 없습니다.
plugtype-optional-hint = 선택되지 않은 상태로 제공되며 사용자가 결정합니다.
plugtype-recommended-hint = 선택된 상태로 제공되며 사용자가 해제할 수 있습니다.
plugtype-not-usable-hint = 회색으로 표시되며 선택할 수 없습니다.
plugtype-could-be-usable-hint = 선택할 수 있지만 설치 프로그램이 작동하지 않을 수 있다고 경고합니다.
op-and = 모든 조건(AND)
op-or = 하나 이상의 조건(OR)
theme-dark = 어둡게
theme-light = 밝게
theme-system = 시스템 설정 따름
condeditor-setter-loc = { "{step}" }단계 / { "{group}" }그룹 / «{ "{name}" }»
condeditor-pattern-of = «{ "{name}" }»의 패턴 → { "{type}" }
condeditor-visibility-of = { "{step}" }단계 표시 여부
condeditor-cond-set = 조건부 세트 { "{num}" }
condeditor-needs = { "{ctx}" } (필요 = { "{value}" })
condeditor-file-dep = { "{ctx}" }: 파일 «{ "{name}" }» ({ "{state}" })
menu-translate-fomod = FOMOD 번역…
ftr-title = FOMOD 번역
ftr-open-folder = 모드 폴더 열기…
ftr-from-active = 활성 프로젝트에서
ftr-from-active-hint = 메인 창에 열려 있는 프로젝트의 FOMOD를 번역합니다(먼저 저장해야 합니다).
ftr-no-fomod = 불러온 FOMOD가 없습니다.
ftr-encoding = 원본 파일의 인코딩입니다. 번역된 파일도 같은 인코딩으로 기록됩니다.
ftr-source-lang = 원본 언어
ftr-target-lang = 대상 언어
ftr-lang-locked = (FOMOD를 불러온 후에는 언어를 바꿀 수 없습니다)
ftr-translator = 번역자:
ftr-save = 번역 저장
ftr-export = 번역된 파일 내보내기
ftr-export-sibling = fomod_<언어> 폴더로
ftr-export-sibling-hint = 번역된 info.xml과 ModuleConfig.xml을 원본 fomod 폴더 옆에 기록합니다. 원본 파일은 변경되지 않습니다.
ftr-export-inplace = 원본 파일에 덮어쓰기
ftr-export-inplace-hint = 각 파일의 타임스탬프가 붙은 .bak 사본을 만든 뒤 fomod/info.xml과 fomod/ModuleConfig.xml을 교체합니다.
ftr-force-explicit-order = 원래 순서 유지
ftr-warn-order = 이름순으로 정렬되는 목록(order="Ascending")은 모드 관리자에서 번역된 이름을 기준으로 다시 정렬됩니다. 이 옵션은 order="Explicit"을 강제하여 옵션이 현재 순서를 유지하도록 합니다.
ftr-update = 폴더에서 업데이트
ftr-update-hint = 디스크에서 FOMOD를 다시 읽어 번역과 병합합니다. 새로 추가된 문자열, 변경된 문자열, 제거된 문자열을 알려 줍니다.
ftr-preview-translated = 번역된 미리보기
ftr-progress = { $done } / { $total } 번역됨
ftr-filter-all = 전체
ftr-filter-untranslated = 미번역
ftr-filter-review = 검토 필요
ftr-filter-issues = 문제 있음
ftr-filter-locked = 잠김
ftr-type-all = 모든 필드
ftr-type-names = 이름
ftr-type-descriptions = 설명
ftr-type-meta = 모드 정보
ftr-search-hint = 원문, 번역 또는 컨텍스트 검색…
ftr-next-untranslated = 다음 미번역 항목
ftr-show-whitespace = 공백과 줄 바꿈 표시
ftr-discard-question = 현재 번역에 저장되지 않은 편집 내용이 있습니다. 이를 버리고 다른 FOMOD를 불러오시겠습니까?
ftr-discard-yes = 버리기
ftr-unsaved-close = 번역에 저장되지 않은 편집 내용이 있습니다.
ftr-col-num = #
ftr-col-status = { "" }
ftr-col-context = 컨텍스트
ftr-col-source = 원문
ftr-col-target = 번역
ftr-col-issues = { "" }
ftr-empty-hint = 모드 폴더를 열거나 활성 프로젝트를 불러오면 번역 가능한 문자열이 나열됩니다.
ftr-empty-filter = 현재 필터와 일치하는 문자열이 없습니다.
ftr-select-row = 번역을 편집할 행을 선택하세요.
ftr-copy-source = 원문 복사
ftr-clear-target = 지우기
ftr-lock = 번역하지 않음
ftr-lock-hint = 잠긴 문자열은 그대로 기록됩니다(제작자, 웹사이트, 고유 명사 등).
ftr-note = 메모:
ftr-status-untranslated = 미번역
ftr-status-translated = 번역됨
ftr-status-auto = 자동으로 채워짐 — 검토해 주세요
ftr-status-fuzzy = 번역 이후 원문이 변경됨 — 검토해 주세요
ftr-status-obsolete = FOMOD에 더 이상 없음
ftr-status-locked = 잠김(그대로 기록됨)
ftr-field-info-name = 모드 이름 (info.xml)
ftr-field-module-name = 설치 프로그램 제목 (ModuleConfig.xml)
ftr-field-author = 제작자
ftr-field-website = 웹사이트
ftr-field-description = 모드 설명
ftr-field-step = 단계 이름
ftr-field-group = 그룹 이름
ftr-field-plugin = 옵션 이름
ftr-field-plugin-desc = 옵션 설명
ftr-issue-empty = 번역이 비어 있음
ftr-issue-whitespace = 번역에 공백만 있음
ftr-issue-edge-whitespace = 앞뒤 공백이 원문과 다름
ftr-issue-token = 보호된 토큰이 다름 — 누락: { $missing } ; 초과: { $extra }
ftr-issue-newline-name = 이름에는 줄 바꿈을 넣을 수 없음
ftr-issue-control = XML에 저장할 수 없는 문자가 포함됨
ftr-issue-length = 원문에 비해 길이가 비정상적임 (×{ $ratio })
ftr-issue-identical = 원문과 동일함
ftr-issue-duplicate = 같은 원문이 { $key }에서 다르게 번역됨
ftr-issue-cdata = 여기에는 ]]> 시퀀스를 사용할 수 없음
ftr-load-error = FOMOD를 불러올 수 없습니다: { $error }
ftr-extracted = 번역 가능한 문자열 { $num }개를 찾았습니다.
ftr-sidecar-found = 기존 번역을 불러와 병합했습니다: 신규 { $new }, 변경 { $changed }, 제거 { $removed }.
ftr-saved = 번역이 { $path }에 저장되었습니다
ftr-save-error = 번역을 저장할 수 없습니다: { $error }
ftr-save-first = 먼저 프로젝트를 저장한 다음 번역하세요.
ftr-export-success = 문자열 { $count }개를 { $path }에 기록했습니다
ftr-export-error = 내보내기 실패: { $error }
ftr-export-blocked = 내보내기 전에 차단 문제 { $num }개를 해결해야 합니다.
ftr-export-stale = FOMOD가 변경되어 문자열 { $num }개를 건너뛰었습니다. “폴더에서 업데이트”를 사용하세요.
ftr-update-report = 업데이트됨: 신규 { $new }, 변경 { $changed }, 이동 { $moved }, 제거 { $removed }, 변경 없음 { $unchanged }.
menu-edit = 편집
menu-undo = 실행 취소
menu-redo = 다시 실행
tree-title = 프로젝트
tree-mod-info = 모드 정보
tree-steps = 설치 단계
tree-required = 필수 파일
tree-conditional = 조건부 설치 항목
tree-empty-steps = 아직 단계가 없습니다 — +를 클릭하여 추가하세요.
tree-duplicate = 복제
tree-delete = 삭제
tree-save-template = 템플릿으로 저장…
tree-drop-hint = 여기에 놓아 이동
cond-set-label = 조건부 세트 { $num }
inspector-empty = 프로젝트 트리에서 항목을 선택하거나 단계를 추가하여 시작하세요.
count-options = 옵션 { $num }개
count-files = 파일 { $num }개
msg-deleted-undo = 삭제되었습니다. 실행 취소(Ctrl+Z)로 복원할 수 있습니다.
problems-title = 문제
problems-errors = 오류 { $num }개
problems-warnings = 경고 { $num }개
btn-close = 닫기
ftr-export-package = 번역 패키지로 (아카이브)
ftr-export-package-hint = 업로드할 수 있는 .zip 또는 .7z를 만듭니다. 번역된 info.xml과 ModuleConfig.xml에 README를 더한 것(패치 전용) 또는 번역된 파일이 포함된 모드 전체(전체)입니다.
ftr-package-full = 모드 전체
ftr-package-full-hint = 번역된 XML 파일 두 개뿐 아니라 모드의 모든 파일을 아카이브에 포함합니다. 제작자가 재배포를 허용하는지 확인하세요.
ftr-package-name-template = 이름:
ftr-readme-patch = 이 아카이브에는 “{ $name }” 설치 프로그램의 번역({ $langname })이 들어 있습니다(fomod/info.xml 및 fomod/ModuleConfig.xml). 원본 모드 위에 설치하거나 모드 관리자가 병합하도록 하여 번역된 파일이 원본 파일을 대체하게 하세요. 설치 프로그램의 텍스트만 바뀌며, 모드 파일 자체는 포함되어 있지 않습니다. XIMOD Architect로 제작되었습니다.
ftr-readme-full = 이 아카이브에는 설치 프로그램이 번역된({ $langname }) “{ $name }” 모드가 들어 있습니다(fomod/info.xml 및 fomod/ModuleConfig.xml). 원본 모드와 같은 방법으로 설치하세요. 설치 프로그램의 텍스트만 변경되었습니다. XIMOD Architect로 제작되었습니다.
ftr-apply-memory = 메모리에서 채우기
ftr-memory-size = 번역 메모리: 이 언어 쌍의 항목 { $num }개. 저장된 모든 번역이 여기에 추가됩니다.
ftr-memory-applied = 번역 메모리에서 문자열 { $num }개를 채웠습니다(“검토 필요”로 표시).
ftr-memory-suggestion = 메모리 제안:
ftr-use-suggestion = 사용
ftr-propagate = 동일한 문자열에 적용
ftr-propagate-hint = 원문이 같고 아직 번역되지 않은 다른 모든 문자열에 이 번역을 복사합니다.
ftr-propagated = 동일한 문자열 { $num }개를 채웠습니다.
ftr-csv-export = CSV 내보내기…
ftr-csv-import = CSV 가져오기…
ftr-csv-imported = CSV 파일에서 문자열 { $num }개를 업데이트했습니다.
ftr-csv-error = CSV 오류: { $error }
ftr-glossary = 용어집
ftr-glossary-source = 용어
ftr-glossary-target = 번역
ftr-glossary-case = 대소문자
ftr-glossary-dnt = 유지
ftr-glossary-add = 용어 추가
ftr-issue-glossary = 용어집: “{ $term }” 용어가 예상대로 번역되지 않았습니다

# ---- Lot F1: open from archive, import fidelity ----
menu-open-archive = 아카이브 열기…
filter-archive = 모드 아카이브 (zip, 7z)
msg-archive-opened = 아카이브를 열었습니다 (파일 { $num }개 추출됨): { $path }
msg-archive-reused = 아카이브가 이미 추출되어 있어 { $path }을(를) 재사용합니다
msg-archive-unsupported = 아카이브 형식 “.{ $ext }”은(는) 지원되지 않습니다. 먼저 7-Zip으로 추출하세요 (.zip과 .7z만 열 수 있습니다).
msg-archive-error = 아카이브를 여는 중 오류가 발생했습니다: { $error }
msg-archive-no-fomod = 아카이브에서 “fomod” 폴더를 찾을 수 없습니다 ({ $path })
msg-archive-extracting = 아카이브를 추출하는 중…
ftr-open-archive = 모드 아카이브 열기…
ftr-package-full-partial = 이 모드는 fomod 폴더만 포함된 아카이브에서 열렸습니다. 전체 패키지에는 추출된 모드가 필요합니다.
info-module-deps = 모드 요구 사항
info-module-deps-hint = 설치 프로그램이 실행되기 전에 모드 전체가 요구하는 파일 또는 플래그 (moduleDependencies). 없으면 비워 두세요.
info-header-advanced = 고급 헤더
info-title-position = 제목 위치
info-title-colour = 제목 색상
info-title-colour-hint = 예상 형식: 16진수 6자리 (RRGGBB)
info-image-show = 헤더 이미지 표시
info-image-fade = 헤더 이미지 페이드
info-image-height = 헤더 이미지 높이
info-attr-default = (기본값)
file-always-install = 항상
file-always-install-hint = 옵션이 선택되지 않았더라도 이 파일을 항상 설치합니다 (alwaysInstall).
file-install-if-usable = 사용 가능 시
file-install-if-usable-hint = 옵션이 선택되지 않았더라도 사용 가능할 때마다 이 파일을 설치합니다 (installIfUsable).
msg-import-lossy = 이 FOMOD에는 XIMOD가 편집할 수 없는 구조가 { $num }개 포함되어 있습니다. 프로젝트를 저장하면 삭제됩니다.
fidelity-nested-deps = { $context }에 중첩된 종속성 그룹이 있습니다 (한 단계만 지원됨)
fidelity-game-dep = { $context }에 게임 버전 요구 사항 { $version }이(가) 있습니다
fidelity-fomm-dep = { $context }에 모드 관리자 버전 요구 사항 { $version }이(가) 있습니다
fidelity-unknown = “{ $parent }” 안의 요소 “{ $element }”은(는) 지원되지 않습니다 ({ $context })
loc-module = 모드 요구 사항
loc-step = { $step }단계 “{ $name }”
loc-installer = 설치 프로그램

# ---- Lot F2: rotating backups, plugin masters, light-plugin checks ----
menu-restore-backup = 백업 복원…
backups-title = 백업 복원
backups-empty = 이 프로젝트에는 아직 백업이 없습니다. 이전 버전 위에 프로젝트를 저장할 때마다 백업이 생성됩니다.
backups-changes = 현재 프로젝트와의 차이 { $num }개
btn-compare = 비교
btn-restore = 복원
btn-delete-backups = 모든 백업 삭제
btn-delete-backups-confirm = 모든 백업을 삭제하려면 다시 클릭하세요
msg-backup-restored = { $time } 백업을 편집기에 복원했습니다 (아직 저장되지 않음; 실행 취소로 되돌릴 수 있음)
msg-backups-deleted = 백업 { $num }개 삭제됨
settings-backup-count = 보관할 백업 수:
settings-backup-count-hint = 저장 시 fomod/backups에 보관되는 FOMOD XML의 이전 버전 수 (0 = 백업 없음).
settings-autosave-minutes = 복구 사본 자동 저장 간격 (분):
settings-autosave-minutes-hint = 이 간격마다 수정된 모든 프로젝트의 복구 사본이 설정 폴더에 기록됩니다. 다음 시작 시 비정상 종료 후에만 제안됩니다 (0 = 끔).
settings-auto-masters = 플러그인의 마스터를 조건으로 추가
settings-auto-masters-hint = 플러그인(.esp/.esm/.esl)을 옵션에 추가하면, 해당 플러그인이 요구하는 마스터 중 게임과 이 모드가 모두 제공하지 않는 것이 옵션의 “Active” 파일 조건이 됩니다.
msg-author-from-plugin = 플러그인 헤더에서 제작자를 채웠습니다: { $author }
msg-masters-added = { $plugin }의 마스터 { $num }개를 파일 조건으로 추가했습니다
issue-missing-master = { $plugin }에는 { $master }이(가) 필요하지만, 이 모드에 없고 종속성으로도 선언되지 않았습니다
issue-esl-mismatch-flag = { $plugin }의 확장자는 .esl이지만 light (ESL) 플래그가 설정되어 있지 않습니다
issue-esl-eligible = { $plugin }은(는) light로 플래그를 지정할 수 있습니다 (새 레코드 { $num }개, 한도 { $limit })
issue-esl-too-big = { $plugin }은(는) light로 플래그가 지정되어 있지만 light 플러그인 규칙에 맞지 않습니다 (새 레코드 { $num }개, 한도 { $limit }, 또는 허용 범위를 벗어난 FormID)
menu-plugin-report = 플러그인 보고서…
plugins-title = 플러그인 보고서
plugins-file = 파일
plugins-kind = 종류
plugins-light = Light 플래그
plugins-masters = 마스터
plugins-new-records = 새 레코드 / 한도
plugins-eligible = Light 가능
plugins-empty = 이 프로젝트는 플러그인 파일(.esp, .esm 또는 .esl)을 설치하지 않습니다.
plugins-unreadable = 읽을 수 없음

# --- V2 lot F3: richer simulator, install sizes, Nexus description ---
preview-tree = 최종 파일 트리
preview-total-size = 총 설치 크기: { $size }
preview-tree-truncated = 트리가 잘렸습니다: 펼칠 파일이 너무 많습니다 (위의 크기는 일부만 반영됨).
preview-overwritten-by = { $plugin }에 의해 덮어쓰기됨
preview-scenario = 시나리오:
preview-scenario-load = 불러오기
preview-scenario-save = 저장…
preview-scenario-delete = 삭제
preview-scenario-name = 시나리오 이름
preview-scenario-saved = 시나리오 "{ $name }"을(를) fomod/scenarios에 저장했습니다
preview-scenario-unresolved = 시나리오의 선택 { $num }개가 이 프로젝트의 어떤 옵션과도 일치하지 않습니다 (이름 변경 또는 제거됨)
preview-scenario-none = (시나리오 없음)
issue-unreachable-step = 단계 "{ $step }"은(는) 절대 표시될 수 없습니다: 표시 조건이 이전의 어떤 옵션도 설정하지 않는 플래그 값을 검사합니다
issue-unreachable-option = 옵션 "{ $plugin }"은(는) 절대 선택될 수 없습니다: 사용 가능 유형 패턴이 어떤 옵션도 설정하지 않는 플래그 값을 검사합니다
issue-unreachable-cond = 조건부 파일 세트 { $num }은(는) 절대 적용될 수 없습니다: 조건이 어떤 옵션도 설정하지 않는 플래그 값을 검사합니다
size-option = 설치 크기: { $size } (파일 { $num }개)
size-missing = 누락된 소스 { $num }개
size-unknown = 설치 크기: — (측정하려면 유효성 검사를 실행하세요)
menu-nexus-desc = Nexus 설명…
nexus-title = Nexus Mods 설명
nexus-format = 형식:
nexus-include-requirements = 요구 사항
nexus-include-options = 설치 옵션
nexus-include-install = 설치
nexus-include-changelog = 변경 내역
nexus-previous = 이전 버전…
nexus-previous-none = (이전 버전 없음: 변경 내역 없음)
nexus-language = 언어:
nexus-language-source = (원본)
nexus-sec-requirements = 요구 사항
nexus-sec-options = 설치 옵션
nexus-sec-install = 설치
nexus-sec-changelog = 변경 내역
nexus-install-text = 이 모드는 FOMOD 설치 프로그램과 함께 제공됩니다: 모드 관리자(Vortex, Mod Organizer 2)로 설치하고 설치 프로그램에서 옵션을 선택하세요.
nexus-requires = 필요
nexus-step = 단계
nexus-added = 추가됨
nexus-removed = 제거됨
nexus-changed = 변경됨
btn-copy = 복사
btn-save-as = 다른 이름으로 저장…
msg-copied = 클립보드에 복사했습니다
msg-saved-to = { $path }에 저장했습니다

# --- V2 lot G1: editing condition editor, project strings ---
condeditor-rename = 이름 바꾸기…
condeditor-rename-exists = "{ $name }"(이)라는 이름의 플래그가 이미 있습니다
condeditor-renamed = 플래그 "{ $from }"의 이름을 "{ $to }"(으)로 바꿨습니다 ({ $num }곳)
condeditor-delete-uses = 모든 사용 삭제
condeditor-deleted-uses = 플래그 "{ $name }"을(를) 모든 곳에서 제거했습니다 ({ $num }곳)
condeditor-values-set = 설정되는 값:
condeditor-values-tested = 검사되는 값:
condeditor-value-never-set = { $value } — 검사되지만 설정된 적 없음
condeditor-value-never-tested = { $value } — 설정되지만 검사된 적 없음
condeditor-builder = 조건 빌더
condeditor-builder-none = 메인 창에서 단계, 옵션, 조건부 파일 세트 또는 모드 정보를 선택하면 여기에서 해당 조건을 편집할 수 있습니다.
condeditor-builder-pattern = 패턴:
condeditor-sentence-if = 만약
condeditor-sentence-and = 그리고
condeditor-sentence-or = 또는
condeditor-sentence-flag = 플래그 { "{name}" } = { "{value}" }
condeditor-sentence-file = 파일 { "{name}" }이(가) { "{value}" }
condeditor-sentence-empty = (조건 없음: 항상 참)
condeditor-sentence-then-visible = 그러면 단계가 표시됩니다
condeditor-sentence-then-type = 그러면 옵션이 { $type }이(가) 됩니다
condeditor-sentence-then-install = 그러면 파일이 설치됩니다
condeditor-sentence-then-module = 그러면 설치 프로그램을 실행할 수 있습니다 (시작 전에 확인됨)
issue-flag-value-never-set = 플래그 "{ $flag }"이(가) 값 "{ $value }"(으)로 검사되지만 어떤 옵션도 이 값을 설정하지 않습니다
issue-flag-never-used = 플래그 "{ $flag }"이(가) 설정되지만 어디에서도 검사되지 않습니다
menu-project-strings = 프로젝트 문자열…
strings-title = 프로젝트 문자열
strings-search = 텍스트, 위치 또는 키 검색…
strings-kind-all = 전체
strings-kind-names = 이름
strings-kind-descriptions = 설명
strings-duplicates-only = 중복만
strings-replace-with = 바꿀 내용:
strings-case = 대/소문자 구분
strings-whole-word = 단어 단위
strings-replace-current = 바꾸기
strings-replace-all = 모두 바꾸기
strings-replaced = 문자열 { $num }개를 바꿨습니다
strings-dup-badge = ×{ $num }
strings-dup-hover = 같은 텍스트:
strings-count = 문자열 { $num }개 · 중복 그룹 { $dups }개
strings-col-location = 위치
strings-col-field = 필드
strings-col-text = 텍스트

# Lot G2 — Bethesda archive readers (BSA/BA2)
menu-archive-contents = 아카이브 내용…
filter-bethesda-archive = Bethesda 아카이브 (bsa, ba2)
archive-view-title = 아카이브 내용
archive-view-format = 형식:
archive-view-entries = 항목 { $num }개
archive-view-size = 압축 해제 시 { $size }
archive-view-search = 경로 검색…
archive-view-col-path = 경로
archive-view-col-size = 크기
archive-view-col-compressed = 압축됨
archive-view-truncated = 일치하는 항목 중 처음 { $num }개만 표시됩니다 — 검색 범위를 좁히세요.
archive-view-error = 이 아카이브를 읽을 수 없습니다: { $error }
archive-view-hint = 이 아카이브의 내용 보기
issue-conflict-archive = 여러 아카이브에 동일한 에셋: “{ $path }”이(가) { $count }개 참조({ $locs })에 의해 패킹되어 있습니다 — 어느 것이 사용될지는 게임의 아카이브 로드 순서가 결정합니다.
issue-conflict-archive-loose = 아카이브 대 루즈 파일: “{ $path }”이(가) 아카이브에 패킹되어 있으면서 루즈 파일로도 설치됩니다({ $locs }) — 루즈 파일이 아카이브 내 파일보다 우선합니다.
preview-in-archive = (아카이브 내)
preview-archived-size = 그중 { $size }는 아카이브에 패킹됨

# --- Project tree: expand / collapse menus
tree-expand = 펼치기
tree-collapse = 접기
tree-expand-all = 모두 펼치기
tree-expand-selected = 선택 항목 펼치기
tree-expand-from = 선택 항목부터 펼치기
tree-collapse-all = 모두 접기
tree-collapse-selected = 선택 항목 접기
tree-collapse-from = 선택 항목부터 접기
tree-expand-all-hint = 모든 제목을 펼칩니다
tree-expand-selected-hint = 선택한 제목만 펼칩니다
tree-expand-from-hint = 선택한 제목과 그 아래의 모든 항목을 펼칩니다
tree-collapse-all-hint = 모든 제목을 접습니다
tree-collapse-selected-hint = 선택한 제목만 접습니다
tree-collapse-from-hint = 선택한 제목과 그 아래의 모든 항목을 접습니다

# --- Lot N: nested dependency groups, version conditions
btn-add-group-cond = 그룹 추가
btn-remove-group-cond = 그룹 제거
dep-type-game = 게임 버전
dep-type-fomm = 모드 관리자 버전
dep-group-hint = 그리고 / 또는으로 결합된 조건 그룹입니다. 그룹은 중첩할 수 있습니다.
condeditor-sentence-game = 게임 버전 ≥ { "{value}" }
condeditor-sentence-fomm = 모드 관리자 버전 ≥ { "{value}" }

# --- FOMOD translator: unique texts mode
ftr-uniques = 고유 텍스트
ftr-uniques-hint = 서로 다른 원문마다 한 행만 표시합니다. 해당 행을 번역하면 같은 원문을 가진 모든 문자열이 한 번에 번역됩니다.
ftr-uniques-synced = 동일한 문자열 { $num }개를 업데이트했습니다.
ftr-uniques-group = 문자열 { $num }개가 이 원문을 공유합니다. 번역은 모두에 적용됩니다.
