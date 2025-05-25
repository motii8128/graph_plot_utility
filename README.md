# graph_plot_utility

|name|status|
|:--:|:--:|
|Rust|[![Rust](https://github.com/motii8128/graph_plot_utility/actions/workflows/rust.yml/badge.svg)](https://github.com/motii8128/graph_plot_utility/actions/workflows/rust.yml)|

# Usage
```toml
[dependencies]
graph_plot_utility = {git = "https://github.com/motii8128/graph_plot_utility.git"}
```

# 使用例１
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
以下のように**result.svg**が出力されます

![](./result.svg)

# 使用例２

つぎに点群を描画する場合の例を示します
```rs
use graph_plot_utility::Plotter;


fn main() {
    // 初期化
    // ｘの範囲は-3.0 ~ 3.0
    // yの範囲は-3.0 ~ 3.0
    // xは0.01ごとに描画するが今回は関係ない
    let mut plotter = Plotter::new(-3.0, 3.0, 0.01, 3.0);

    // ベクトルで(x,y)で点群を描画できる
    let mut points = Vec::<(f64,f64)>::new();

    // (x,y) = (-2.0, -2.0)の点をベクトルに追加
    points.push((-2.0, -2.0));
    points.push((-1.0, -1.0));
    points.push((0.0, 0.0));
    points.push((1.0, 1.0));

    // プロッターに点群を追加
    plotter.add_points(points);

    plotter.save("./result_from_points.svg");
}

```

![](./result_from_points.svg)
