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
menu-open = 폴더 열기...
menu-open-file = 파일 열기...
menu-save = 저장
menu-recent = 최근 항목
menu-exit = 종료
menu-merge = FOMOD 병합...
menu-export = 배포 아카이브 내보내기...

# Menu - Options
menu-options = 옵션
menu-settings = 설정
menu-pre-save-script = 저장 전 스크립트...
menu-post-save-script = 저장 후 스크립트...
menu-translation = 번역...

# Menu - Help
menu-help = 도움말
menu-about = 정보

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
label-plugin-name = 플러그인 이름:
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
btn-add-plugin = 플러그인 추가
btn-remove-plugin = 플러그인 제거
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
validation-no-plugins = 단계 { $step }, 그룹 "{ $name }"에 플러그인이 없습니다

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
default-plugin-name = 플러그인 { $num }
pattern-label = 패턴 { $num }

# Selection prompts
msg-select-group-first = 먼저 그룹을 선택하십시오.
msg-select-plugin-edit = 편집할 플러그인을 선택하십시오.
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
