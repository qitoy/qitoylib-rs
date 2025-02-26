macro_rules! mod_libs {
    ($mod:ident; $($lib:ident)* ) => {
        pub mod $mod {
            $(
                #[doc(inline)]
                pub use $lib::*;
            )*
        }
    }
}

mod_libs!(algebra;
    qitoy_group
    qitoy_ring
);
mod_libs!(algorithm;
    qitoy_mo
    qitoy_rerooting_dp
);
mod_libs!(automaton;
    qitoy_dfa
    qitoy_dfa_leq
    qitoy_dfa_leq_inv
    qitoy_dfa_multiple_of
    qitoy_dfa_non_zero
    qitoy_nfa
);
mod_libs!(data_structure;
    qitoy_potentialized_unionfind
    qitoy_red_black_tree
);
mod_libs!(fps;
    qitoy_berlekamp_massey
    qitoy_bostan_mori
);
mod_libs!(math;
    qitoy_combi
    qitoy_matrix
);
mod_libs!(prime;
    qitoy_prime_check
    qitoy_prime_factorize
    qitoy_prime_sieve
    qitoy_prime_pi
);
mod_libs!(proc;
    qitoy_derive
);
mod_libs!(string;
    qitoy_rolling_hash
);
mod_libs!(utils;
    qitoy_utils_float2uint
);
