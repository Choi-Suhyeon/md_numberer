# MD_NUMBERER

Herein lies a program that provides numbering for each heading within a markdown file.
마크다운 파일 내의 각 제목에 대해 넘버링을 해주는 프로그램입니다.

## USEAGE GUIDELINES
```
sample : md_numberer --file sample.md -l 2 -s 1

syntax :
    md_numberer --help | -h
    md_numberer --file <file_name> [-l <header_number_limit>] [-s <starting_number>]
    md_numberer --directory <directory_name> [-l <header_number_limit>] [-s <starting_number>]
```

- `-l` : 넘버링을 시작할 헤드 번호. 입력 범위 : [1, 6]. 기본값 : 1
- `-s` : 넘버링을 시작할 번호. 입력 범위 : [0, 1]. 기본값 : 1
 
## TO-DO LIST
1. ~~`h1`부터 `h6` 중 어디부터 넘버링을 할지 옵션 넣기.~~
1. ~~넘버링을 1부터 시작할지 0부터 시작할지 옵션 넣기.~~
1. ~~폴더를 지정하면 폴더 내 모든 마크다운 파일에 대해 넘버링을 적용시키는 옵션 넣기.~~
