mod array_ex;

fn main() {
    array_ex::array_scores(2, 500);
    array_ex::while_array();
    array_ex::days_of_week(1);
    array_ex::calc_average();

    let color = array_ex::new_color(255, 0, 0);
    color.print_color();
}
