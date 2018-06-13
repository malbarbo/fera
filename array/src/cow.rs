use {BalancedShiftSplit, NestedArray, RcArray};

pub type CowNestedArray<T, S = BalancedShiftSplit> =
    NestedArray<T, S, RcArray<RcArray<T>>, RcArray<T>>;

#[cfg(test)]
mod tests {
    use {ArrayTests, CowNestedArray, DynamicArrayTests};

    struct T;

    impl ArrayTests for T {
        type A = CowNestedArray<usize>;
    }

    delegate_tests!{
        T,
        basic_0,
        basic_001k,
        basic_100k,
        clone_001k,
        clone_100k
    }

    impl DynamicArrayTests for T {}

    delegate_tests!{
        T,
        capacity,
        push_1k,
        clone_push
    }

    #[cfg(all(feature = "nightly", test))]
    mod benchs {
        use super::T;
        use test::Bencher;
        use ArrayBenchs;

        impl ArrayBenchs for T {}

        delegate_benchs!{
            T,
            fold_xor_0001k,
            fold_xor_0010k,
            fold_xor_0100k,
            fold_xor_1000k,
            clone_change_0001k,
            clone_change_0010k,
            clone_change_0100k,
            clone_change_1000k
        }
    }
}
