# fast-reader

可以快速在大文件里面进行逐行的关键字查询，替换等操作。注意：以行为单位，会用一些符号进行分割。

功能：
1. 查询关键行

```
  xxx -f splite matched_idx matched_key expore_idx file
```

2. 替换功能

```
  xxx -r file.conf file
```

file.conf format

```
{
  match_file<optional>: {
    file_name: "",
    comps: [], # idxs
    matched_idx: xx,
    begin_idx: xx,<optional default 0>
    end_idx: xx,<optional default end>
    split: "",<optional default " ">
  },
  ori_file: {
    comps: [],
    matched_idx: xx,
    begin_idx: xx,<optional default 0>
    end_idx: xx,<optional default end>
    split: "",<optional default " ">
  }
}
```
