pub trait IsWide {
    fn is_wide(&self) -> bool;
    fn width(&self) -> u32 {
        if self.is_wide() {
            2
        } else {
            1
        }
    }
}

impl IsWide for char {
    fn is_wide(&self) -> bool {
        if '\u{1100}' <= *self && *self <= '\u{115f}' {
            return true;
        }
        if '\u{231a}' <= *self && *self <= '\u{231b}' {
            return true;
        }
        if *self == '\u{2329}' {
            return true;
        }
        if *self == '\u{232a}' {
            return true;
        }
        if '\u{23e9}' <= *self && *self <= '\u{23ec}' {
            return true;
        }
        if *self == '\u{23f0}' {
            return true;
        }
        if *self == '\u{23f3}' {
            return true;
        }
        if '\u{25fd}' <= *self && *self <= '\u{25fe}' {
            return true;
        }
        if '\u{2614}' <= *self && *self <= '\u{2615}' {
            return true;
        }
        if '\u{2648}' <= *self && *self <= '\u{2653}' {
            return true;
        }
        if *self == '\u{267f}' {
            return true;
        }
        if *self == '\u{2693}' {
            return true;
        }
        if *self == '\u{26a1}' {
            return true;
        }
        if '\u{26aa}' <= *self && *self <= '\u{26ab}' {
            return true;
        }
        if '\u{26bd}' <= *self && *self <= '\u{26be}' {
            return true;
        }
        if '\u{26c4}' <= *self && *self <= '\u{26c5}' {
            return true;
        }
        if *self == '\u{26ce}' {
            return true;
        }
        if *self == '\u{26d4}' {
            return true;
        }
        if *self == '\u{26ea}' {
            return true;
        }
        if '\u{26f2}' <= *self && *self <= '\u{26f3}' {
            return true;
        }
        if *self == '\u{26f5}' {
            return true;
        }
        if *self == '\u{26fa}' {
            return true;
        }
        if *self == '\u{26fd}' {
            return true;
        }
        if *self == '\u{2705}' {
            return true;
        }
        if '\u{270a}' <= *self && *self <= '\u{270b}' {
            return true;
        }
        if *self == '\u{2728}' {
            return true;
        }
        if *self == '\u{274c}' {
            return true;
        }
        if *self == '\u{274e}' {
            return true;
        }
        if '\u{2753}' <= *self && *self <= '\u{2755}' {
            return true;
        }
        if *self == '\u{2757}' {
            return true;
        }
        if '\u{2795}' <= *self && *self <= '\u{2797}' {
            return true;
        }
        if *self == '\u{27b0}' {
            return true;
        }
        if *self == '\u{27bf}' {
            return true;
        }
        if '\u{2b1b}' <= *self && *self <= '\u{2b1c}' {
            return true;
        }
        if *self == '\u{2b50}' {
            return true;
        }
        if *self == '\u{2b55}' {
            return true;
        }
        if '\u{2e80}' <= *self && *self <= '\u{2e99}' {
            return true;
        }
        if '\u{2e9b}' <= *self && *self <= '\u{2ef3}' {
            return true;
        }
        if '\u{2f00}' <= *self && *self <= '\u{2fd5}' {
            return true;
        }
        if '\u{2ff0}' <= *self && *self <= '\u{2fff}' {
            return true;
        }
        if *self == '\u{3000}' {
            return true;
        }
        if '\u{3001}' <= *self && *self <= '\u{3003}' {
            return true;
        }
        if *self == '\u{3004}' {
            return true;
        }
        if *self == '\u{3005}' {
            return true;
        }
        if *self == '\u{3006}' {
            return true;
        }
        if *self == '\u{3007}' {
            return true;
        }
        if *self == '\u{3008}' {
            return true;
        }
        if *self == '\u{3009}' {
            return true;
        }
        if *self == '\u{300a}' {
            return true;
        }
        if *self == '\u{300b}' {
            return true;
        }
        if *self == '\u{300c}' {
            return true;
        }
        if *self == '\u{300d}' {
            return true;
        }
        if *self == '\u{300e}' {
            return true;
        }
        if *self == '\u{300f}' {
            return true;
        }
        if *self == '\u{3010}' {
            return true;
        }
        if *self == '\u{3011}' {
            return true;
        }
        if '\u{3012}' <= *self && *self <= '\u{3013}' {
            return true;
        }
        if *self == '\u{3014}' {
            return true;
        }
        if *self == '\u{3015}' {
            return true;
        }
        if *self == '\u{3016}' {
            return true;
        }
        if *self == '\u{3017}' {
            return true;
        }
        if *self == '\u{3018}' {
            return true;
        }
        if *self == '\u{3019}' {
            return true;
        }
        if *self == '\u{301a}' {
            return true;
        }
        if *self == '\u{301b}' {
            return true;
        }
        if *self == '\u{301c}' {
            return true;
        }
        if *self == '\u{301d}' {
            return true;
        }
        if '\u{301e}' <= *self && *self <= '\u{301f}' {
            return true;
        }
        if *self == '\u{3020}' {
            return true;
        }
        if '\u{3021}' <= *self && *self <= '\u{3029}' {
            return true;
        }
        if '\u{302a}' <= *self && *self <= '\u{302d}' {
            return true;
        }
        if '\u{302e}' <= *self && *self <= '\u{302f}' {
            return true;
        }
        if *self == '\u{3030}' {
            return true;
        }
        if '\u{3031}' <= *self && *self <= '\u{3035}' {
            return true;
        }
        if '\u{3036}' <= *self && *self <= '\u{3037}' {
            return true;
        }
        if '\u{3038}' <= *self && *self <= '\u{303a}' {
            return true;
        }
        if *self == '\u{303b}' {
            return true;
        }
        if *self == '\u{303c}' {
            return true;
        }
        if *self == '\u{303d}' {
            return true;
        }
        if *self == '\u{303e}' {
            return true;
        }
        if '\u{3041}' <= *self && *self <= '\u{3096}' {
            return true;
        }
        if '\u{3099}' <= *self && *self <= '\u{309a}' {
            return true;
        }
        if '\u{309b}' <= *self && *self <= '\u{309c}' {
            return true;
        }
        if '\u{309d}' <= *self && *self <= '\u{309e}' {
            return true;
        }
        if *self == '\u{309f}' {
            return true;
        }
        if *self == '\u{30a0}' {
            return true;
        }
        if '\u{30a1}' <= *self && *self <= '\u{30fa}' {
            return true;
        }
        if *self == '\u{30fb}' {
            return true;
        }
        if '\u{30fc}' <= *self && *self <= '\u{30fe}' {
            return true;
        }
        if *self == '\u{30ff}' {
            return true;
        }
        if '\u{3105}' <= *self && *self <= '\u{312f}' {
            return true;
        }
        if '\u{3131}' <= *self && *self <= '\u{318e}' {
            return true;
        }
        if '\u{3190}' <= *self && *self <= '\u{3191}' {
            return true;
        }
        if '\u{3192}' <= *self && *self <= '\u{3195}' {
            return true;
        }
        if '\u{3196}' <= *self && *self <= '\u{319f}' {
            return true;
        }
        if '\u{31a0}' <= *self && *self <= '\u{31bf}' {
            return true;
        }
        if '\u{31c0}' <= *self && *self <= '\u{31e3}' {
            return true;
        }
        if *self == '\u{31ef}' {
            return true;
        }
        if '\u{31f0}' <= *self && *self <= '\u{31ff}' {
            return true;
        }
        if '\u{3200}' <= *self && *self <= '\u{321e}' {
            return true;
        }
        if '\u{3220}' <= *self && *self <= '\u{3229}' {
            return true;
        }
        if '\u{322a}' <= *self && *self <= '\u{3247}' {
            return true;
        }
        if *self == '\u{3250}' {
            return true;
        }
        if '\u{3251}' <= *self && *self <= '\u{325f}' {
            return true;
        }
        if '\u{3260}' <= *self && *self <= '\u{327f}' {
            return true;
        }
        if '\u{3280}' <= *self && *self <= '\u{3289}' {
            return true;
        }
        if '\u{328a}' <= *self && *self <= '\u{32b0}' {
            return true;
        }
        if '\u{32b1}' <= *self && *self <= '\u{32bf}' {
            return true;
        }
        if '\u{32c0}' <= *self && *self <= '\u{32ff}' {
            return true;
        }
        if '\u{3300}' <= *self && *self <= '\u{33ff}' {
            return true;
        }
        if '\u{3400}' <= *self && *self <= '\u{4dbf}' {
            return true;
        }
        if '\u{4e00}' <= *self && *self <= '\u{9fff}' {
            return true;
        }
        if '\u{a000}' <= *self && *self <= '\u{a014}' {
            return true;
        }
        if *self == '\u{a015}' {
            return true;
        }
        if '\u{a016}' <= *self && *self <= '\u{a48c}' {
            return true;
        }
        if '\u{a490}' <= *self && *self <= '\u{a4c6}' {
            return true;
        }
        if '\u{a960}' <= *self && *self <= '\u{a97c}' {
            return true;
        }
        if '\u{ac00}' <= *self && *self <= '\u{d7a3}' {
            return true;
        }
        if '\u{f900}' <= *self && *self <= '\u{fa6d}' {
            return true;
        }
        if '\u{fa6e}' <= *self && *self <= '\u{fa6f}' {
            return true;
        }
        if '\u{fa70}' <= *self && *self <= '\u{fad9}' {
            return true;
        }
        if '\u{fada}' <= *self && *self <= '\u{faff}' {
            return true;
        }
        if '\u{fe10}' <= *self && *self <= '\u{fe16}' {
            return true;
        }
        if *self == '\u{fe17}' {
            return true;
        }
        if *self == '\u{fe18}' {
            return true;
        }
        if *self == '\u{fe19}' {
            return true;
        }
        if *self == '\u{fe30}' {
            return true;
        }
        if '\u{fe31}' <= *self && *self <= '\u{fe32}' {
            return true;
        }
        if '\u{fe33}' <= *self && *self <= '\u{fe34}' {
            return true;
        }
        if *self == '\u{fe35}' {
            return true;
        }
        if *self == '\u{fe36}' {
            return true;
        }
        if *self == '\u{fe37}' {
            return true;
        }
        if *self == '\u{fe38}' {
            return true;
        }
        if *self == '\u{fe39}' {
            return true;
        }
        if *self == '\u{fe3a}' {
            return true;
        }
        if *self == '\u{fe3b}' {
            return true;
        }
        if *self == '\u{fe3c}' {
            return true;
        }
        if *self == '\u{fe3d}' {
            return true;
        }
        if *self == '\u{fe3e}' {
            return true;
        }
        if *self == '\u{fe3f}' {
            return true;
        }
        if *self == '\u{fe40}' {
            return true;
        }
        if *self == '\u{fe41}' {
            return true;
        }
        if *self == '\u{fe42}' {
            return true;
        }
        if *self == '\u{fe43}' {
            return true;
        }
        if *self == '\u{fe44}' {
            return true;
        }
        if '\u{fe45}' <= *self && *self <= '\u{fe46}' {
            return true;
        }
        if *self == '\u{fe47}' {
            return true;
        }
        if *self == '\u{fe48}' {
            return true;
        }
        if '\u{fe49}' <= *self && *self <= '\u{fe4c}' {
            return true;
        }
        if '\u{fe4d}' <= *self && *self <= '\u{fe4f}' {
            return true;
        }
        if '\u{fe50}' <= *self && *self <= '\u{fe52}' {
            return true;
        }
        if '\u{fe54}' <= *self && *self <= '\u{fe57}' {
            return true;
        }
        if *self == '\u{fe58}' {
            return true;
        }
        if *self == '\u{fe59}' {
            return true;
        }
        if *self == '\u{fe5a}' {
            return true;
        }
        if *self == '\u{fe5b}' {
            return true;
        }
        if *self == '\u{fe5c}' {
            return true;
        }
        if *self == '\u{fe5d}' {
            return true;
        }
        if *self == '\u{fe5e}' {
            return true;
        }
        if '\u{fe5f}' <= *self && *self <= '\u{fe61}' {
            return true;
        }
        if *self == '\u{fe62}' {
            return true;
        }
        if *self == '\u{fe63}' {
            return true;
        }
        if '\u{fe64}' <= *self && *self <= '\u{fe66}' {
            return true;
        }
        if *self == '\u{fe68}' {
            return true;
        }
        if *self == '\u{fe69}' {
            return true;
        }
        if '\u{fe6a}' <= *self && *self <= '\u{fe6b}' {
            return true;
        }
        if '\u{ff01}' <= *self && *self <= '\u{ff03}' {
            return true;
        }
        if *self == '\u{ff04}' {
            return true;
        }
        if '\u{ff05}' <= *self && *self <= '\u{ff07}' {
            return true;
        }
        if *self == '\u{ff08}' {
            return true;
        }
        if *self == '\u{ff09}' {
            return true;
        }
        if *self == '\u{ff0a}' {
            return true;
        }
        if *self == '\u{ff0b}' {
            return true;
        }
        if *self == '\u{ff0c}' {
            return true;
        }
        if *self == '\u{ff0d}' {
            return true;
        }
        if '\u{ff0e}' <= *self && *self <= '\u{ff0f}' {
            return true;
        }
        if '\u{ff10}' <= *self && *self <= '\u{ff19}' {
            return true;
        }
        if '\u{ff1a}' <= *self && *self <= '\u{ff1b}' {
            return true;
        }
        if '\u{ff1c}' <= *self && *self <= '\u{ff1e}' {
            return true;
        }
        if '\u{ff1f}' <= *self && *self <= '\u{ff20}' {
            return true;
        }
        if '\u{ff21}' <= *self && *self <= '\u{ff3a}' {
            return true;
        }
        if *self == '\u{ff3b}' {
            return true;
        }
        if *self == '\u{ff3c}' {
            return true;
        }
        if *self == '\u{ff3d}' {
            return true;
        }
        if *self == '\u{ff3e}' {
            return true;
        }
        if *self == '\u{ff3f}' {
            return true;
        }
        if *self == '\u{ff40}' {
            return true;
        }
        if '\u{ff41}' <= *self && *self <= '\u{ff5a}' {
            return true;
        }
        if *self == '\u{ff5b}' {
            return true;
        }
        if *self == '\u{ff5c}' {
            return true;
        }
        if *self == '\u{ff5d}' {
            return true;
        }
        if *self == '\u{ff5e}' {
            return true;
        }
        if *self == '\u{ff5f}' {
            return true;
        }
        if *self == '\u{ff60}' {
            return true;
        }
        if '\u{ffe0}' <= *self && *self <= '\u{ffe1}' {
            return true;
        }
        if *self == '\u{ffe2}' {
            return true;
        }
        if *self == '\u{ffe3}' {
            return true;
        }
        if *self == '\u{ffe4}' {
            return true;
        }
        if '\u{ffe5}' <= *self && *self <= '\u{ffe6}' {
            return true;
        }
        if '\u{16fe0}' <= *self && *self <= '\u{16fe1}' {
            return true;
        }
        if *self == '\u{16fe2}' {
            return true;
        }
        if *self == '\u{16fe3}' {
            return true;
        }
        if *self == '\u{16fe4}' {
            return true;
        }
        if '\u{16ff0}' <= *self && *self <= '\u{16ff1}' {
            return true;
        }
        if '\u{17000}' <= *self && *self <= '\u{187f7}' {
            return true;
        }
        if '\u{18800}' <= *self && *self <= '\u{18aff}' {
            return true;
        }
        if '\u{18b00}' <= *self && *self <= '\u{18cd5}' {
            return true;
        }
        if '\u{18d00}' <= *self && *self <= '\u{18d08}' {
            return true;
        }
        if '\u{1aff0}' <= *self && *self <= '\u{1aff3}' {
            return true;
        }
        if '\u{1aff5}' <= *self && *self <= '\u{1affb}' {
            return true;
        }
        if '\u{1affd}' <= *self && *self <= '\u{1affe}' {
            return true;
        }
        if '\u{1b000}' <= *self && *self <= '\u{1b0ff}' {
            return true;
        }
        if '\u{1b100}' <= *self && *self <= '\u{1b122}' {
            return true;
        }
        if *self == '\u{1b132}' {
            return true;
        }
        if '\u{1b150}' <= *self && *self <= '\u{1b152}' {
            return true;
        }
        if *self == '\u{1b155}' {
            return true;
        }
        if '\u{1b164}' <= *self && *self <= '\u{1b167}' {
            return true;
        }
        if '\u{1b170}' <= *self && *self <= '\u{1b2fb}' {
            return true;
        }
        if *self == '\u{1f004}' {
            return true;
        }
        if *self == '\u{1f0cf}' {
            return true;
        }
        if *self == '\u{1f18e}' {
            return true;
        }
        if '\u{1f191}' <= *self && *self <= '\u{1f19a}' {
            return true;
        }
        if '\u{1f200}' <= *self && *self <= '\u{1f202}' {
            return true;
        }
        if '\u{1f210}' <= *self && *self <= '\u{1f23b}' {
            return true;
        }
        if '\u{1f240}' <= *self && *self <= '\u{1f248}' {
            return true;
        }
        if '\u{1f250}' <= *self && *self <= '\u{1f251}' {
            return true;
        }
        if '\u{1f260}' <= *self && *self <= '\u{1f265}' {
            return true;
        }
        if '\u{1f300}' <= *self && *self <= '\u{1f320}' {
            return true;
        }
        if '\u{1f32d}' <= *self && *self <= '\u{1f335}' {
            return true;
        }
        if '\u{1f337}' <= *self && *self <= '\u{1f37c}' {
            return true;
        }
        if '\u{1f37e}' <= *self && *self <= '\u{1f393}' {
            return true;
        }
        if '\u{1f3a0}' <= *self && *self <= '\u{1f3ca}' {
            return true;
        }
        if '\u{1f3cf}' <= *self && *self <= '\u{1f3d3}' {
            return true;
        }
        if '\u{1f3e0}' <= *self && *self <= '\u{1f3f0}' {
            return true;
        }
        if *self == '\u{1f3f4}' {
            return true;
        }
        if '\u{1f3f8}' <= *self && *self <= '\u{1f3fa}' {
            return true;
        }
        if '\u{1f3fb}' <= *self && *self <= '\u{1f3ff}' {
            return true;
        }
        if '\u{1f400}' <= *self && *self <= '\u{1f43e}' {
            return true;
        }
        if *self == '\u{1f440}' {
            return true;
        }
        if '\u{1f442}' <= *self && *self <= '\u{1f4fc}' {
            return true;
        }
        if '\u{1f4ff}' <= *self && *self <= '\u{1f53d}' {
            return true;
        }
        if '\u{1f54b}' <= *self && *self <= '\u{1f54e}' {
            return true;
        }
        if '\u{1f550}' <= *self && *self <= '\u{1f567}' {
            return true;
        }
        if *self == '\u{1f57a}' {
            return true;
        }
        if '\u{1f595}' <= *self && *self <= '\u{1f596}' {
            return true;
        }
        if *self == '\u{1f5a4}' {
            return true;
        }
        if '\u{1f5fb}' <= *self && *self <= '\u{1f5ff}' {
            return true;
        }
        if '\u{1f600}' <= *self && *self <= '\u{1f64f}' {
            return true;
        }
        if '\u{1f680}' <= *self && *self <= '\u{1f6c5}' {
            return true;
        }
        if *self == '\u{1f6cc}' {
            return true;
        }
        if '\u{1f6d0}' <= *self && *self <= '\u{1f6d2}' {
            return true;
        }
        if '\u{1f6d5}' <= *self && *self <= '\u{1f6d7}' {
            return true;
        }
        if '\u{1f6dc}' <= *self && *self <= '\u{1f6df}' {
            return true;
        }
        if '\u{1f6eb}' <= *self && *self <= '\u{1f6ec}' {
            return true;
        }
        if '\u{1f6f4}' <= *self && *self <= '\u{1f6fc}' {
            return true;
        }
        if '\u{1f7e0}' <= *self && *self <= '\u{1f7eb}' {
            return true;
        }
        if *self == '\u{1f7f0}' {
            return true;
        }
        if '\u{1f90c}' <= *self && *self <= '\u{1f93a}' {
            return true;
        }
        if '\u{1f93c}' <= *self && *self <= '\u{1f945}' {
            return true;
        }
        if '\u{1f947}' <= *self && *self <= '\u{1f9ff}' {
            return true;
        }
        if '\u{1fa70}' <= *self && *self <= '\u{1fa7c}' {
            return true;
        }
        if '\u{1fa80}' <= *self && *self <= '\u{1fa88}' {
            return true;
        }
        if '\u{1fa90}' <= *self && *self <= '\u{1fabd}' {
            return true;
        }
        if '\u{1fabf}' <= *self && *self <= '\u{1fac5}' {
            return true;
        }
        if '\u{1face}' <= *self && *self <= '\u{1fadb}' {
            return true;
        }
        if '\u{1fae0}' <= *self && *self <= '\u{1fae8}' {
            return true;
        }
        if '\u{1faf0}' <= *self && *self <= '\u{1faf8}' {
            return true;
        }
        if '\u{20000}' <= *self && *self <= '\u{2a6df}' {
            return true;
        }
        if '\u{2a6e0}' <= *self && *self <= '\u{2a6ff}' {
            return true;
        }
        if '\u{2a700}' <= *self && *self <= '\u{2b739}' {
            return true;
        }
        if '\u{2b73a}' <= *self && *self <= '\u{2b73f}' {
            return true;
        }
        if '\u{2b740}' <= *self && *self <= '\u{2b81d}' {
            return true;
        }
        if '\u{2b81e}' <= *self && *self <= '\u{2b81f}' {
            return true;
        }
        if '\u{2b820}' <= *self && *self <= '\u{2cea1}' {
            return true;
        }
        if '\u{2cea2}' <= *self && *self <= '\u{2ceaf}' {
            return true;
        }
        if '\u{2ceb0}' <= *self && *self <= '\u{2ebe0}' {
            return true;
        }
        if '\u{2ebe1}' <= *self && *self <= '\u{2ebef}' {
            return true;
        }
        if '\u{2ebf0}' <= *self && *self <= '\u{2ee5d}' {
            return true;
        }
        if '\u{2ee5e}' <= *self && *self <= '\u{2f7ff}' {
            return true;
        }
        if '\u{2f800}' <= *self && *self <= '\u{2fa1d}' {
            return true;
        }
        if '\u{2fa1e}' <= *self && *self <= '\u{2fa1f}' {
            return true;
        }
        if '\u{2fa20}' <= *self && *self <= '\u{2fffd}' {
            return true;
        }
        if '\u{30000}' <= *self && *self <= '\u{3134a}' {
            return true;
        }
        if '\u{3134b}' <= *self && *self <= '\u{3134f}' {
            return true;
        }
        if '\u{31350}' <= *self && *self <= '\u{323af}' {
            return true;
        }
        if '\u{323b0}' <= *self && *self <= '\u{3fffd}' {
            return true;
        }
        false
    }
}
