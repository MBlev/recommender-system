pub mod distances;
pub mod movie_api;
pub mod book_api;
pub mod small_movielens_api;

fn main() {
    movie_api::distance_by_name(String::from("Heather"), String::from("Patrick C"), distances::Distance::Manhattan);
    movie_api::distance_by_id(String::from("1"), String::from("2"), distances::Distance::Manhattan);
    book_api::distance_by_id(String::from("26182"), String::from("37400"), distances::Distance::Manhattan);
    small_movielens_api::distance_by_id(String::from("125"), String::from("567"), distances::Distance::Manhattan);

    println!("\n");

    movie_api::distance_by_name(String::from("Heather"), String::from("Patrick C"), distances::Distance::Euclidean);
    movie_api::distance_by_id(String::from("1"), String::from("2"), distances::Distance::Euclidean);
    book_api::distance_by_id(String::from("26182"), String::from("37400"), distances::Distance::Euclidean);
    small_movielens_api::distance_by_id(String::from("125"), String::from("567"), distances::Distance::Euclidean);

    println!("\n");

    movie_api::distance_by_name(String::from("Heather"), String::from("Patrick C"), distances::Distance::Minkowski(3));
    movie_api::distance_by_id(String::from("1"), String::from("2"), distances::Distance::Minkowski(3));
    book_api::distance_by_id(String::from("26182"), String::from("37400"), distances::Distance::Minkowski(3));
    small_movielens_api::distance_by_id(String::from("125"), String::from("567"), distances::Distance::Minkowski(3));

    println!("\n");

    movie_api::distance_by_name(String::from("Heather"), String::from("Patrick C"), distances::Distance::Pearson);
    movie_api::distance_by_id(String::from("1"), String::from("2"), distances::Distance::Pearson);
    book_api::distance_by_id(String::from("26182"), String::from("37400"), distances::Distance::Pearson);
    small_movielens_api::distance_by_id(String::from("125"), String::from("567"), distances::Distance::Pearson);

    println!("\n");

    movie_api::distance_by_name(String::from("Heather"), String::from("Patrick C"), distances::Distance::Cosine);
    movie_api::distance_by_id(String::from("1"), String::from("2"), distances::Distance::Cosine);
    book_api::distance_by_id(String::from("26182"), String::from("37400"), distances::Distance::Cosine);
    small_movielens_api::distance_by_id(String::from("125"), String::from("567"), distances::Distance::Cosine);

    println!("\n");

    movie_api::distance_by_name(String::from("Heather"), String::from("Patrick C"), distances::Distance::JaccardDist);
    movie_api::distance_by_id(String::from("1"), String::from("2"), distances::Distance::JaccardDist);
    book_api::distance_by_id(String::from("26182"), String::from("37400"), distances::Distance::JaccardDist);
    small_movielens_api::distance_by_id(String::from("125"), String::from("567"), distances::Distance::JaccardDist);

    println!("\n");

    movie_api::distance_by_name(String::from("Heather"), String::from("Patrick C"), distances::Distance::JaccardSim);
    movie_api::distance_by_id(String::from("1"), String::from("2"), distances::Distance::JaccardSim);
    book_api::distance_by_id(String::from("26182"), String::from("37400"), distances::Distance::JaccardSim);
    small_movielens_api::distance_by_id(String::from("125"), String::from("567"), distances::Distance::JaccardSim);
}
