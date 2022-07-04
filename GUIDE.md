# Git 에 대한 이해

## Git Style Guide 숙지

[ikaruce/git-style-guide](https://github.com/ikaruce/git-style-guide)

## 터미널 환경 설정

- Windows
    - WSL2를 공부하고 설치.
    - Windows terminal 설치.
    - Git 명령어 수행은 CLI 환경인 terminal에서 진행.
    - [WSL란?](https://docs.microsoft.com/ko-kr/windows/wsl/about)
    - [WSL 설치 및 WSL 2로 업데이트](https://docs.microsoft.com/ko-kr/windows/wsl/install-win10)
- Mac
    - iterm2 설치 후 iterm 터미널에서 진행
    - `brew install --cask iterm2`

## 시작하기

### 새 저장소 만들고 시작하기

```bash
mkdir <생성하고 싶은 디렉토리 이름>
cd <생성한 디렉토리 이름>
git init

# ex)
~ $ mkdir test
~ $ cd test
~/test $ git init
```

### 원격 저장소를 가져와서 시작하기

```bash
git clone <원격 저장소 주소>

# 예) git clone https://github.com/wholemann/daily-coding-dojo.git
```


## 원격 저장소 추가하기

### 원격 저장소 목록 확인

```bash
git remote
```

### 원격 저장소 정보 자세히 보기

```bash
git remote -v
```


### origin 원격 저장소 추가

주의) clone으로 원격 저장소를 가져오면 origin 원격 저장소가 이미 추가된 상태입니다.

아래는 git init 후 수동으로 원격 저장소를 추가할 때 사용하는 방법입니다.

```bash
git remote add origin <내 계정의 repository 주소>

git fetch origin
```

### upstream 원격 저장소 추가

```bash
git remote add upstream <PR을 보낼 원격 repository 주소>

git fetch upstream
```

## Pull Request를 이용한 협업 시작하기

- upstream - PR을 보내고 싶은 repository(회사에선 우리팀 repository)
- origin - upstream에서 내 계정으로 fork한 repository


### Step #0

먼저 PR(Pull Request)를 보내고 싶은 repository를 fork 합니다.
fork를 하면 본인 계정의 GitHub에 동일한 repository가 복사됩니다.

내 계정에 동일한 repository가 복제된 걸 확인할 수 있습니다. 초록색 `Code` 버튼을 눌러 나오는 링크를 복사합니다.

복사한 주소를 이용하면 내 로컬 머신에서 clone 할 수 있습니다.

주의) clone으로 원격 저장소를 가져오면 origin 원격 저장소가 이미 추가된 상태입니다.

```bash
git clone <내 계정에 fork된 repository 주소>

git remote add upstream <PR을 보낼 원격 repository 주소>
```

PR을 보내는 과정에서 원격 저장소는 upstream. origin 2개가 필요합니다.
로컬 머신(내 컴퓨터) 관점에서 보면 내 계정의 repository도 원격이고,
upstream의 repository도 원격입니다.

- upstream - PR을 보내고 싶은 repository(회사에선 우리팀 repository)
- origin - upstream에서 내 계정으로 fork한 repository

`git remote -v` 를 통해 upstream과 origin이 아래와 같은지 확인합니다.

```bash
~/IdeaProjects/near $ git remote -v
origin  https://github.com/happyhappy-jun/near.git (fetch)
origin  https://github.com/happyhappy-jun/near.git (push)
upstream        https://github.com/postech-dao/near.git (fetch)
upstream        https://github.com/postech-dao/near.git (push)
```


### Step #1 - 작업 브랜치 만들기

```bash
git switch -c <브랜치 이름> upstream/main
```

upstream/main는 붙여서 쓰고, 가운데 슬래시(/)가 들어갑니다.

브랜치 이름은 작업한 내용을 표현해야 합니다. ex) login-oauth-apply

### Step #2 - upstream 원격 저장소의 최신 상태를 반영하기

#### git fetch

[fetch(가져오기)](https://backlog.com/git-tutorial/kr/stepup/stepup3_2.html)

#### git rebase

[rebase로 병합하기](https://backlog.com/git-tutorial/kr/stepup/stepup2_8.html)

```bash
git fetch upstream
```

### Step #3 - 작업하기

원하는 작업을 이 시점에 합니다.

### Step #4 - 커밋

뭔가 바뀐 점을 추가합니다.

새 파일을 추가하는 게 아니라, 파일 추가/변경/삭제란 “바뀐 점”을 추가합니다.

```bash
git add .
```

방금 추가한 바뀐 점을 커밋합니다.

가능하면 메시지를 우리가 나중에 찾아볼 수 있는 형태로 씁니다.

```bash
git commit
```


### Step #5 - origin 원격 저장소에 작업 브랜치 올리기

```bash
git push origin <브랜치 이름>
```

origin과 <브랜치 이름> 사이엔 공백이 들어갑니다.

### Step #6 - Pull Request

GitHub에서 New Pull Request를 합니다.
