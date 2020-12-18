# STD for Marrow runtime

- [ ] 优化二进制文件大小
  - [ ] 去掉std alloc 和wee_alloc的使用
    1. 避免使用Rc与VecDeque -> 使用自定义链表.
    2. 避免使用Box -> &'a dyn Trait.自己做生命周期标注.
- [X] 优化API
  - [X] 避免提供全局spawn_local;
- [ ] 增加SQLite与HTTP接口
  - [ ] SQLite.
  - [ ] HTTP.