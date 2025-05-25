# graph_plot_utility

|name|status|
|:--:|:--:|
|Rust|[![Rust](https://github.com/motii8128/graph_plot_utility/actions/workflows/rust.yml/badge.svg)](https://github.com/motii8128/graph_plot_utility/actions/workflows/rust.yml)|

# Usage
```toml
[dependencies]
graph_plot_utility = {git = "https://github.com/motii8128/graph_plot_utility.git"}
```

# 使用例
２つの関数をグラフに描く例
```rs
use graph_plot_utility::Plotter;

// eのx乗
fn f(x:f64)->f64
{
    std::f64::consts::E.powf(x)
}

// x + 1.0
fn f2(x:f64)->f64
{
    x + 1.0
}

fn main() {
    // 初期化
    // ｘの最小値、ｘの最大値、ｘの間隔、ｙの最大値（絶対値）
    let mut plotter = Plotter::new(-10.0, 10.0, 0.01, 10.0);

    // 関数fを引数として渡す
    plotter.add_func(f);

    // 関数f2を引数として渡す
    plotter.add_func(f2);

    // .svgで保存
    plotter.save("./result.svg");
}
```



