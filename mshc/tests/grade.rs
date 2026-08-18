#[cfg(feature = "grade-a2e")]
mod grade_a2e {
    use mshc::grade::Grade;

    #[test]
    fn grade_a2e_caps() {
        assert_eq!(Grade::A, Grade::floor());
        assert_eq!(Grade::E, Grade::ceil());
    }
}

#[cfg(all(feature = "grade-f-to-sss", not(feature = "grade-a2e")))]
mod grade_f_to_sss {
    use mshc::grade::Grade;

    #[test]
    fn grade_f2sss_caps() {
        assert_eq!(Grade::F, Grade::floor());
        assert_eq!(Grade::SSS, Grade::ceil());
    }
}
