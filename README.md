# 试试库操作系统

```bash
cargo qemu --plat <PLAT> --app <APP>
```

- `APP` 可用项：
  - `helloworld`
  - `echo`

- `PLAT` 可用项：
  - `qemu-virt`
  - `qemu-sifive_u`

## 依赖传递的方式

| --------- | 性能敏感 | 性能不敏感
| --------- | ------- | -
| 使用广泛   | 静态链接 | 动态注入
| 使用不广泛 | feature | feature
