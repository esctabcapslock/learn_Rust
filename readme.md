# Rust 공부하기

- 다음 [튜토리얼](https://doc.rust-lang.org/stable/book)을 참조한다
- 다음 [번역...](https://rinthel.github.io/rust-lang-book-ko/)이 더 좋은 것 같다



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
git commit -m "잔디오류"
git push origin master
```

```cmd
git reset HEAD 
```

### 잔디 심기 오류 있음

- [잔디](https://txegg.tistory.com/107)처럼, 내 이메일도 약간 잘못되어 있었다...
- `git config --global --list`로 확인하고 `git config --global user.email "[mail@mail.com]" `통해 바꾸자.
