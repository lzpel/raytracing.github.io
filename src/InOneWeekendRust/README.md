# 2025/06/18

- raytracing.github.io をフォーク
- とりあえずビルドし実行してみた 
  - image1.png
  - image2.png
  - image3.png
- 方針決定：Rustに移植する
  - raytracing.github.io/src/InOneWeekendの隣にraytracing.github.io/src/InOneWeekendRustを置いた
- InOneWeekendRust
  - src/InOneWeekendRust/src/ray.rs:
    - bookとソースを参考に完成