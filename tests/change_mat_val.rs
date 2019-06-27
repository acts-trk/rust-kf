use kalman_rs::config::*;
#[macro_use]
use kalman_rs::change_mat_val;

fn init_mat() -> (Mat3, usize) {
    (Mat3::zeros(), 3)
}

fn init_mxn() -> (Mat5x2, usize) {
    (Mat5x2::zeros(), 5)
}

// tests for N x N matricies
mod square_mat {
    use super::*;

    #[test]
    fn mat_test_1() {
        let (mut mat, dim) = init_mat();
        let new_val = 10.0;
        change_mat_val!{mat: dim;
            [0,0] => new_val           // very first index
        }

        dbg!{&mat};

        assert_eq!(
            *mat.get(0).expect("out of bounds"),
            new_val
        )
    }

    #[test]
    fn mat_test_2() {
        let (mut mat, dim) = init_mat();
        let new_val = 10.0;
        change_mat_val!{mat: dim;
            [2,2] => 10.0            // very last index
        }

        dbg!{&mat};

        assert_eq!(
            *mat.get(8).expect("out of bounds"),
            new_val
        )
    }


    #[test]
    fn mat_test_3() {
        let (mut mat, dim) = init_mat();
        let new_val = 10.0;
        change_mat_val!{mat: dim;
            [2,0] => 10.0           // bottom left corner
        }

        dbg!{&mat};

        assert_eq!(
            *mat.get(2).expect("out of bounds"),
            new_val
        )
    }



    #[test]
    fn mat_test_4() {
        let (mut mat, dim) = init_mat();
        let new_val = 10.0;
        change_mat_val!{mat: dim;
            [0,2] => 10.0       // top right corner
        }

        dbg!{&mat};

        assert_eq!(
            *mat.get(6).expect("out of bounds"),
            new_val
        )
    }
    #[test]
    fn mat_test_5() {
        let (mut mat, dim) = init_mat();
        let new_val = 10.0;
        change_mat_val!{mat: dim;
            [1,0] => 10.0       // top right corner
        }

        dbg!{&mat};

        assert_eq!(
            *mat.get(1).expect("out of bounds"),
            new_val
        )
    }
}

// uses 5x2 mat
mod non_square_mat {
    use super::*;

    #[test]
    fn mxn_mat_test_1() {
        let (mut mat, dim) = init_mxn();
        let new_val = 1.;
        let index = 0;

        change_mat_val!{
            mat: dim;
            [0,0] => new_val
        }

        assert_eq!{
            *mat.get(index).expect("out of bounds"),
            new_val
        }
    }

    #[test]
    fn mxn_mat_test_2() {
        let (mut mat, dim) = init_mxn();
        let new_val = 1.;
        let index = 9;

        change_mat_val!{
            mat: dim;
            [4,1] => new_val
        }

        dbg!{&mat};

        assert_eq!{
            *mat.get(index).expect("out of bounds"),
            new_val
        }
    }

    #[test]
    fn mxn_mat_test_3() {
        let (mut mat, dim) = init_mxn();
        let new_val = 1.;
        let index = 7;

        change_mat_val!{
            mat: dim;
            [2,1] => new_val
        }

        dbg!{&mat};

        assert_eq!{
            *mat.get(index).expect("out of bounds"),
            new_val
        }
    }

    #[test]
    fn mxn_mat_test_4() {
        let (mut mat, dim) = init_mxn();
        let new_val = 1.;
        let index = 3;

        change_mat_val!{
            mat: dim;
            [3,0] => new_val
        }

        dbg!{&mat};

        assert_eq!{
            *mat.get(index).expect("out of bounds"),
            new_val
        }
    }

    #[test]
    fn mxn_mat_test_5() {
        let (mut mat, dim) = init_mxn();
        let new_val = 1.;
        let index = 5;

        change_mat_val!{
            mat: dim;
            [0,1] => new_val
        }

        dbg!{&mat};

        assert_eq!{
            *mat.get(index).expect("out of bounds"),
            new_val
        }
    }

}