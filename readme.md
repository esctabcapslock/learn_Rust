# Rust 공부하기
- 다음 [튜토리얼](https://doc.rust-lang.org/stable/book)을 참조한다
- 다음 [번역](https://rinthel.github.io/rust-lang-book-ko/)이 더 좋은 것 같다
- 다음 [칼럼](https://blog.appleseed.dev/post/fascinated-by-rust-in-a-week/)이 잘 요약

## git
```cmd
git init
git remote add origin https://github.com/esctabcapslock/learn_Rust
```

### 업로드
[add 관련](https://linuxize.com/post/gitignore-ignoring-files-in-git/)
```cmd
git add *
git add ../readme.md
git add ../.gitignore
git status
git commit -m "깃 이멜오류"
git push origin master
```

```cmd
git reset HEAD 
```

### 잔디 심기 오류 있음

- [잔디](https://txegg.tistory.com/107)처럼, 내 이메일도 약간 잘못되어 있었다...
- `git config --global --list`로 확인하고 `git config --global user.email "[mail@mail.com]" `통해 바꾸자.
- [사생활 이멜 오류](https://stackoverflow.com/questions/43863522/error-your-push-would-publish-a-private-email-address)는 이렇게 바꾸자
- 내 id는 [다음](https://stackoverflow.com/questions/17308954/where-can-i-find-the-github-id-in-my-account)을 참조하자.
- `git config --global user.email "{ID}+{username}@users.noreply.github.com"`
- `git config --global user.email "62400002+esctabcapslock@users.noreply.github.com`

## vscode 관련

- `ctrl`+`M`카를 누르면 `tab`키 안먹힘
- `rust-analyzer` 확장이 `Julia` 확장이랑 충돌함.  `Julia` 비활성화 해야 함


## 시작하기
`cmd
$ cargo new --bin p_15_deref
     Created binary (application) `p_12_cli` package
`