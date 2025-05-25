use graph_plot_utility::Plotter;

fn f(x:f64)->f64
{
    std::f64::consts::E.powf(x)
}

fn f2(x:f64)->f64
{
    x + 1.0
}

fn main() {
    let mut plotter = Plotter::new(-10.0, 10.0, 0.01, 10.0);

    plotter.add_func(f);
    plotter.add_func(f2);

    plotter.save("./result.svg");
}
