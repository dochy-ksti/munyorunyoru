mod A {
    fn hoge() -> usize {
        0
    }

    mod C {
        fn hoge() {}
    }

    mod B {
        fn hoge() {
            super::hoge();
            //super::C::hoge();
        }
    }
}

fn hoge() {
    //self::A::hoge();
}
