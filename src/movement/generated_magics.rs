// Generating rook magics across 12 threads (one seed each), keeping the smallest table...
// (only thread 0 prints per-square detail; the rest run silently)
// square  0: bits=12 offset=0 table_len=4096 fill=62.5% (5119498 attempts)
// square  7: bits=12 offset=4096 table_len=4096 fill=100.0% (5051775 attempts)
// square 56: bits=12 offset=8191 table_len=4096 fill=51.6% (5012434 attempts)
// square 63: bits=12 offset=12276 table_len=4096 fill=53.1% (5001928 attempts)
// square  1: bits=11 offset=16371 table_len=2048 fill=74.9% (5015536 attempts)
// square  2: bits=11 offset=18418 table_len=2048 fill=68.8% (5105059 attempts)
// square  3: bits=11 offset=20465 table_len=2048 fill=62.0% (5020169 attempts)
// square  4: bits=11 offset=22506 table_len=2048 fill=62.5% (5032591 attempts)
// square  5: bits=11 offset=24551 table_len=2048 fill=68.8% (5097322 attempts)
// square  6: bits=11 offset=26598 table_len=2048 fill=61.8% (5060681 attempts)
// square  8: bits=11 offset=28646 table_len=2046 fill=62.6% (5002048 attempts)
// square 15: bits=11 offset=30691 table_len=2048 fill=75.0% (5002739 attempts)
// square 16: bits=11 offset=32739 table_len=2048 fill=69.5% (5039991 attempts)
// square 23: bits=11 offset=34787 table_len=2048 fill=75.0% (5025768 attempts)
// square 24: bits=11 offset=36835 table_len=2048 fill=72.9% (5005061 attempts)
// square 31: bits=11 offset=38883 table_len=2048 fill=100.0% (5003504 attempts)
// square 32: bits=11 offset=40931 table_len=2047 fill=62.5% (5016579 attempts)
// square 39: bits=11 offset=42978 table_len=2048 fill=62.5% (5002213 attempts)
// square 40: bits=11 offset=45025 table_len=2048 fill=62.5% (5006088 attempts)
// square 47: bits=11 offset=47072 table_len=2048 fill=56.2% (5034946 attempts)
// square 48: bits=11 offset=49120 table_len=2048 fill=53.1% (5007268 attempts)
// square 55: bits=11 offset=51167 table_len=2048 fill=53.1% (5003941 attempts)
// square 57: bits=11 offset=53185 table_len=2048 fill=51.6% (5008927 attempts)
// square 58: bits=11 offset=55230 table_len=2048 fill=51.6% (5000046 attempts)
// square 59: bits=11 offset=57265 table_len=2048 fill=51.6% (5021450 attempts)
// square 60: bits=11 offset=59278 table_len=2048 fill=56.2% (5017849 attempts)
// square 61: bits=11 offset=61325 table_len=2048 fill=62.5% (5010986 attempts)
// square 62: bits=11 offset=63372 table_len=2048 fill=53.1% (5001603 attempts)
// square  9: bits=10 offset=65420 table_len=1024 fill=70.3% (5036427 attempts)
// square 10: bits=10 offset=66444 table_len=1024 fill=71.9% (5031783 attempts)
// square 11: bits=10 offset=67461 table_len=1024 fill=56.2% (5015848 attempts)
// square 12: bits=10 offset=68483 table_len=1024 fill=62.5% (5011682 attempts)
// square 13: bits=10 offset=69507 table_len=1024 fill=75.0% (5006726 attempts)
// square 14: bits=10 offset=70531 table_len=1024 fill=66.6% (5002301 attempts)
// square 17: bits=10 offset=71555 table_len=1024 fill=67.2% (5009003 attempts)
// square 18: bits=10 offset=72579 table_len=1024 fill=82.8% (5067322 attempts)
// square 19: bits=10 offset=73602 table_len=1024 fill=75.0% (5019214 attempts)
// square 20: bits=10 offset=74626 table_len=1024 fill=100.0% (5302931 attempts)
// square 21: bits=10 offset=75650 table_len=1024 fill=100.0% (5000482 attempts)
// square 22: bits=10 offset=76674 table_len=1024 fill=75.0% (5003202 attempts)
// square 25: bits=10 offset=77698 table_len=1024 fill=67.2% (5050857 attempts)
// square 26: bits=10 offset=78722 table_len=1024 fill=82.8% (5105715 attempts)
// square 27: bits=10 offset=79746 table_len=1024 fill=87.5% (5033266 attempts)
// square 28: bits=10 offset=80770 table_len=1024 fill=75.0% (5053879 attempts)
// square 29: bits=10 offset=81794 table_len=1024 fill=75.0% (5110399 attempts)
// square 30: bits=10 offset=82818 table_len=1024 fill=100.0% (5002471 attempts)
// square 33: bits=10 offset=83842 table_len=1024 fill=72.4% (5000868 attempts)
// square 34: bits=10 offset=84866 table_len=1024 fill=84.4% (5029827 attempts)
// square 35: bits=10 offset=85890 table_len=1024 fill=70.3% (5078551 attempts)
// square 36: bits=10 offset=86914 table_len=1024 fill=75.0% (5026282 attempts)
// square 37: bits=10 offset=87938 table_len=1024 fill=75.0% (5004337 attempts)
// square 38: bits=10 offset=88961 table_len=1024 fill=75.0% (5018732 attempts)
// square 41: bits=10 offset=89982 table_len=1024 fill=56.2% (5006139 attempts)
// square 42: bits=10 offset=91006 table_len=1024 fill=97.7% (5139822 attempts)
// square 43: bits=10 offset=92029 table_len=1024 fill=62.5% (5073193 attempts)
// square 44: bits=10 offset=93053 table_len=1024 fill=62.5% (5038548 attempts)
// square 45: bits=10 offset=94077 table_len=1024 fill=75.0% (5024982 attempts)
// square 46: bits=10 offset=95101 table_len=1022 fill=62.6% (5040248 attempts)
// square 49: bits=10 offset=96122 table_len=1024 fill=53.1% (5000295 attempts)
// square 50: bits=10 offset=97143 table_len=1023 fill=53.2% (5013458 attempts)
// square 51: bits=10 offset=98159 table_len=1024 fill=53.1% (5005526 attempts)
// square 52: bits=10 offset=99183 table_len=1021 fill=53.3% (5004979 attempts)
// square 53: bits=10 offset=100200 table_len=1024 fill=53.1% (5002959 attempts)
// [thread 7] seed 8885303498275828532: table size 798.74 Kb (in 2307.53s)
// [thread 1] seed 15348192573181382059: table size 799.07 Kb (in 2320.86s)
// [thread 6] seed 15881013158765235087: table size 796.30 Kb (in 2323.80s)
// square 54: bits=10 offset=101222 table_len=1024 fill=53.1% (5008774 attempts)
// [thread 0] seed 15512407338438524295: table size 798.80 Kb (in 2323.94s)
// [thread 5] seed 1035986090179171509: table size 798.88 Kb (in 2324.82s)
// [thread 8] seed 9351298888654082800: table size 799.48 Kb (in 2326.44s)
// [thread 11] seed 1781602018382584721: table size 799.02 Kb (in 2326.64s)
// [thread 3] seed 8865639049760002203: table size 799.03 Kb (in 2328.04s)
// [thread 9] seed 16986587351727726250: table size 796.57 Kb (in 2329.44s)
// [thread 2] seed 4036907382994161593: table size 798.20 Kb (in 2330.51s)
// [thread 10] seed 9424380621860533918: table size 798.91 Kb (in 2332.09s)
// [thread 4] seed 12408660808952000385: table size 798.66 Kb (in 2339.43s)
// Best rook global table: 796.30 Kb out of 12 threads (total time 2339.43s)
pub const ROOK_MAGICS: [MagicEntry; 64] = [
    MagicEntry { number: 6953558386969411585, blocker_mask: 0x000101010101017e, inverse_blocker_mask: 0xfffefefefefefe81, offset: 0, index_bits: 12, shift: 52 },
    MagicEntry { number: 1603283941782458371, blocker_mask: 0x000202020202027c, inverse_blocker_mask: 0xfffdfdfdfdfdfd83, offset: 16374, index_bits: 11, shift: 53 },
    MagicEntry { number: 72079721726230532, blocker_mask: 0x000404040404047a, inverse_blocker_mask: 0xfffbfbfbfbfbfb85, offset: 18422, index_bits: 11, shift: 53 },
    MagicEntry { number: 144126186413494304, blocker_mask: 0x0008080808080876, inverse_blocker_mask: 0xfff7f7f7f7f7f789, offset: 20470, index_bits: 11, shift: 53 },
    MagicEntry { number: 10448355606694723840, blocker_mask: 0x001010101010106e, inverse_blocker_mask: 0xffefefefefefef91, offset: 22515, index_bits: 11, shift: 53 },
    MagicEntry { number: 9259405234101289088, blocker_mask: 0x002020202020205e, inverse_blocker_mask: 0xffdfdfdfdfdfdfa1, offset: 24562, index_bits: 11, shift: 53 },
    MagicEntry { number: 9511606970372587558, blocker_mask: 0x004040404040403e, inverse_blocker_mask: 0xffbfbfbfbfbfbfc1, offset: 26605, index_bits: 11, shift: 53 },
    MagicEntry { number: 9367487439834447873, blocker_mask: 0x008080808080807e, inverse_blocker_mask: 0xff7f7f7f7f7f7f81, offset: 4093, index_bits: 12, shift: 52 },
    MagicEntry { number: 1482106492055191564, blocker_mask: 0x0001010101017e00, inverse_blocker_mask: 0xfffefefefefe81ff, offset: 28650, index_bits: 11, shift: 53 },
    MagicEntry { number: 9183190105718792, blocker_mask: 0x0002020202027c00, inverse_blocker_mask: 0xfffdfdfdfdfd83ff, offset: 65387, index_bits: 10, shift: 54 },
    MagicEntry { number: 563062729869440, blocker_mask: 0x0004040404047a00, inverse_blocker_mask: 0xfffbfbfbfbfb85ff, offset: 66410, index_bits: 10, shift: 54 },
    MagicEntry { number: 5066618870841346, blocker_mask: 0x0008080808087600, inverse_blocker_mask: 0xfff7f7f7f7f789ff, offset: 67433, index_bits: 10, shift: 54 },
    MagicEntry { number: 72620566025077248, blocker_mask: 0x0010101010106e00, inverse_blocker_mask: 0xffefefefefef91ff, offset: 68456, index_bits: 10, shift: 54 },
    MagicEntry { number: 3460171924338966529, blocker_mask: 0x0020202020205e00, inverse_blocker_mask: 0xffdfdfdfdfdfa1ff, offset: 69480, index_bits: 10, shift: 54 },
    MagicEntry { number: 1316177060930257032, blocker_mask: 0x0040404040403e00, inverse_blocker_mask: 0xffbfbfbfbfbfc1ff, offset: 70504, index_bits: 10, shift: 54 },
    MagicEntry { number: 144678141049186819, blocker_mask: 0x0080808080807e00, inverse_blocker_mask: 0xff7f7f7f7f7f81ff, offset: 30698, index_bits: 11, shift: 53 },
    MagicEntry { number: 1351365762341929088, blocker_mask: 0x00010101017e0100, inverse_blocker_mask: 0xfffefefefe81feff, offset: 32745, index_bits: 11, shift: 53 },
    MagicEntry { number: 18014536083701769, blocker_mask: 0x00020202027c0200, inverse_blocker_mask: 0xfffdfdfdfd83fdff, offset: 71528, index_bits: 10, shift: 54 },
    MagicEntry { number: 2323268606689290, blocker_mask: 0x00040404047a0400, inverse_blocker_mask: 0xfffbfbfbfb85fbff, offset: 72551, index_bits: 10, shift: 54 },
    MagicEntry { number: 565149249576964, blocker_mask: 0x0008080808760800, inverse_blocker_mask: 0xfff7f7f7f789f7ff, offset: 73575, index_bits: 10, shift: 54 },
    MagicEntry { number: 4786174250029058, blocker_mask: 0x00101010106e1000, inverse_blocker_mask: 0xffefefefef91efff, offset: 74599, index_bits: 10, shift: 54 },
    MagicEntry { number: 1126449734615168, blocker_mask: 0x00202020205e2000, inverse_blocker_mask: 0xffdfdfdfdfa1dfff, offset: 75623, index_bits: 10, shift: 54 },
    MagicEntry { number: 5197580580636590178, blocker_mask: 0x00404040403e4000, inverse_blocker_mask: 0xffbfbfbfbfc1bfff, offset: 76647, index_bits: 10, shift: 54 },
    MagicEntry { number: 6757598556724225, blocker_mask: 0x00808080807e8000, inverse_blocker_mask: 0xff7f7f7f7f817fff, offset: 34793, index_bits: 11, shift: 53 },
    MagicEntry { number: 4630267782120931456, blocker_mask: 0x000101017e010100, inverse_blocker_mask: 0xfffefefe81fefeff, offset: 36840, index_bits: 11, shift: 53 },
    MagicEntry { number: 1161103610480769, blocker_mask: 0x000202027c020200, inverse_blocker_mask: 0xfffdfdfd83fdfdff, offset: 77671, index_bits: 10, shift: 54 },
    MagicEntry { number: 45071184941302017, blocker_mask: 0x000404047a040400, inverse_blocker_mask: 0xfffbfbfb85fbfbff, offset: 78695, index_bits: 10, shift: 54 },
    MagicEntry { number: 1495201681947770912, blocker_mask: 0x0008080876080800, inverse_blocker_mask: 0xfff7f7f789f7f7ff, offset: 79719, index_bits: 10, shift: 54 },
    MagicEntry { number: 324261380785965072, blocker_mask: 0x001010106e101000, inverse_blocker_mask: 0xffefefef91efefff, offset: 80742, index_bits: 10, shift: 54 },
    MagicEntry { number: 9367488328737752064, blocker_mask: 0x002020205e202000, inverse_blocker_mask: 0xffdfdfdfa1dfdfff, offset: 81766, index_bits: 10, shift: 54 },
    MagicEntry { number: 11538503758954758148, blocker_mask: 0x004040403e404000, inverse_blocker_mask: 0xffbfbfbfc1bfbfff, offset: 82790, index_bits: 10, shift: 54 },
    MagicEntry { number: 5643587059780, blocker_mask: 0x008080807e808000, inverse_blocker_mask: 0xff7f7f7f817f7fff, offset: 38888, index_bits: 11, shift: 53 },
    MagicEntry { number: 4683886566223921408, blocker_mask: 0x0001017e01010100, inverse_blocker_mask: 0xfffefe81fefefeff, offset: 40935, index_bits: 11, shift: 53 },
    MagicEntry { number: 567362025718018, blocker_mask: 0x0002027c02020200, inverse_blocker_mask: 0xfffdfd83fdfdfdff, offset: 83814, index_bits: 10, shift: 54 },
    MagicEntry { number: 576465201895833696, blocker_mask: 0x0004047a04040400, inverse_blocker_mask: 0xfffbfb85fbfbfbff, offset: 84805, index_bits: 10, shift: 54 },
    MagicEntry { number: 4611690699975303344, blocker_mask: 0x0008087608080800, inverse_blocker_mask: 0xfff7f789f7f7f7ff, offset: 85797, index_bits: 10, shift: 54 },
    MagicEntry { number: 4611688217886855328, blocker_mask: 0x0010106e10101000, inverse_blocker_mask: 0xffefef91efefefff, offset: 86821, index_bits: 10, shift: 54 },
    MagicEntry { number: 1188950992320730112, blocker_mask: 0x0020205e20202000, inverse_blocker_mask: 0xffdfdfa1dfdfdfff, offset: 87841, index_bits: 10, shift: 54 },
    MagicEntry { number: 281477132583424, blocker_mask: 0x0040403e40404000, inverse_blocker_mask: 0xffbfbfc1bfbfbfff, offset: 88865, index_bits: 10, shift: 54 },
    MagicEntry { number: 144678242452766976, blocker_mask: 0x0080807e80808000, inverse_blocker_mask: 0xff7f7f817f7f7fff, offset: 42982, index_bits: 11, shift: 53 },
    MagicEntry { number: 9223380008457764864, blocker_mask: 0x00017e0101010100, inverse_blocker_mask: 0xfffe81fefefefeff, offset: 45029, index_bits: 11, shift: 53 },
    MagicEntry { number: 3372170633216, blocker_mask: 0x00027c0202020200, inverse_blocker_mask: 0xfffd83fdfdfdfdff, offset: 89884, index_bits: 10, shift: 54 },
    MagicEntry { number: 576463655835672576, blocker_mask: 0x00047a0404040400, inverse_blocker_mask: 0xfffb85fbfbfbfbff, offset: 90907, index_bits: 10, shift: 54 },
    MagicEntry { number: 2346480967604109328, blocker_mask: 0x0008760808080800, inverse_blocker_mask: 0xfff789f7f7f7f7ff, offset: 91929, index_bits: 10, shift: 54 },
    MagicEntry { number: 18015508087504912, blocker_mask: 0x00106e1010101000, inverse_blocker_mask: 0xffef91efefefefff, offset: 92953, index_bits: 10, shift: 54 },
    MagicEntry { number: 285873040064584, blocker_mask: 0x00205e2020202000, inverse_blocker_mask: 0xffdfa1dfdfdfdfff, offset: 93976, index_bits: 10, shift: 54 },
    MagicEntry { number: 280387141644, blocker_mask: 0x00403e4040404000, inverse_blocker_mask: 0xffbfc1bfbfbfbfff, offset: 94999, index_bits: 10, shift: 54 },
    MagicEntry { number: 36028870301900802, blocker_mask: 0x00807e8080808000, inverse_blocker_mask: 0xff7f817f7f7f7fff, offset: 47076, index_bits: 11, shift: 53 },
    MagicEntry { number: 2305852012560155136, blocker_mask: 0x007e010101010100, inverse_blocker_mask: 0xff81fefefefefeff, offset: 49109, index_bits: 11, shift: 53 },
    MagicEntry { number: 1273526100112, blocker_mask: 0x007c020202020200, inverse_blocker_mask: 0xff83fdfdfdfdfdff, offset: 96017, index_bits: 10, shift: 54 },
    MagicEntry { number: 360290457378488512, blocker_mask: 0x007a040404040400, inverse_blocker_mask: 0xff85fbfbfbfbfbff, offset: 96900, index_bits: 10, shift: 54 },
    MagicEntry { number: 2305844457863512576, blocker_mask: 0x0076080808080800, inverse_blocker_mask: 0xff89f7f7f7f7f7ff, offset: 97871, index_bits: 10, shift: 54 },
    MagicEntry { number: 1189513252406856192, blocker_mask: 0x006e101010101000, inverse_blocker_mask: 0xff91efefefefefff, offset: 98889, index_bits: 10, shift: 54 },
    MagicEntry { number: 72058695160316000, blocker_mask: 0x005e202020202000, inverse_blocker_mask: 0xffa1dfdfdfdfdfff, offset: 99911, index_bits: 10, shift: 54 },
    MagicEntry { number: 74309531467124768, blocker_mask: 0x003e404040404000, inverse_blocker_mask: 0xffc1bfbfbfbfbfff, offset: 100906, index_bits: 10, shift: 54 },
    MagicEntry { number: 4035225407925670400, blocker_mask: 0x007e808080808000, inverse_blocker_mask: 0xff817f7f7f7f7fff, offset: 51156, index_bits: 11, shift: 53 },
    MagicEntry { number: 554084391442, blocker_mask: 0x7e01010101010100, inverse_blocker_mask: 0x81fefefefefefeff, offset: 8188, index_bits: 12, shift: 52 },
    MagicEntry { number: 4899916673761161250, blocker_mask: 0x7c02020202020200, inverse_blocker_mask: 0x83fdfdfdfdfdfdff, offset: 53204, index_bits: 11, shift: 53 },
    MagicEntry { number: 279443444762, blocker_mask: 0x7a04040404040400, inverse_blocker_mask: 0x85fbfbfbfbfbfbff, offset: 55194, index_bits: 11, shift: 53 },
    MagicEntry { number: 144116339270754370, blocker_mask: 0x7608080808080800, inverse_blocker_mask: 0x89f7f7f7f7f7f7ff, offset: 57226, index_bits: 11, shift: 53 },
    MagicEntry { number: 270497453869123585, blocker_mask: 0x6e10101010101000, inverse_blocker_mask: 0x91efefefefefefff, offset: 59250, index_bits: 11, shift: 53 },
    MagicEntry { number: 2814750174563354, blocker_mask: 0x5e20202020202000, inverse_blocker_mask: 0xa1dfdfdfdfdfdfff, offset: 61295, index_bits: 11, shift: 53 },
    MagicEntry { number: 290482210879841796, blocker_mask: 0x3e40404040404000, inverse_blocker_mask: 0xc1bfbfbfbfbfbfff, offset: 63342, index_bits: 11, shift: 53 },
    MagicEntry { number: 144115229148644354, blocker_mask: 0x7e80808080808000, inverse_blocker_mask: 0x817f7f7f7f7f7fff, offset: 12278, index_bits: 12, shift: 52 },
];
// Generating bishop magics across 12 threads (one seed each), keeping the smallest table...
// (only thread 0 prints per-square detail; the rest run silently)
// square  0: bits=6 offset=0 table_len=64 fill=51.6% (5000975 attempts)
// square  7: bits=6 offset=41 table_len=64 fill=51.6% (5000620 attempts)
// square 56: bits=6 offset=97 table_len=64 fill=42.2% (5002112 attempts)
// square 63: bits=6 offset=159 table_len=64 fill=51.6% (5001103 attempts)
// square  1: bits=5 offset=222 table_len=32 fill=53.1% (5000703 attempts)
// square  2: bits=5 offset=254 table_len=32 fill=56.2% (5000350 attempts)
// square  3: bits=5 offset=284 table_len=32 fill=62.5% (5000393 attempts)
// square  4: bits=5 offset=314 table_len=32 fill=62.5% (5000011 attempts)
// square  5: bits=5 offset=339 table_len=32 fill=56.2% (5000016 attempts)
// square  6: bits=5 offset=370 table_len=32 fill=53.1% (5000952 attempts)
// square  8: bits=5 offset=402 table_len=32 fill=53.1% (5000095 attempts)
// square 15: bits=5 offset=413 table_len=32 fill=53.1% (5001437 attempts)
// square 16: bits=5 offset=443 table_len=32 fill=56.2% (5002119 attempts)
// square 23: bits=5 offset=475 table_len=32 fill=56.2% (5000520 attempts)
// square 24: bits=5 offset=507 table_len=32 fill=62.5% (5002759 attempts)
// square 31: bits=5 offset=539 table_len=32 fill=62.5% (5002048 attempts)
// square 32: bits=5 offset=569 table_len=32 fill=62.5% (5000421 attempts)
// square 39: bits=5 offset=601 table_len=32 fill=62.5% (5000140 attempts)
// square 40: bits=5 offset=628 table_len=32 fill=56.2% (5000230 attempts)
// square 47: bits=5 offset=660 table_len=32 fill=56.2% (5001517 attempts)
// square 48: bits=5 offset=684 table_len=30 fill=56.7% (5000134 attempts)
// square 55: bits=5 offset=711 table_len=30 fill=50.0% (5001029 attempts)
// square 57: bits=5 offset=738 table_len=32 fill=53.1% (5000153 attempts)
// square 58: bits=5 offset=770 table_len=32 fill=56.2% (5001564 attempts)
// square 59: bits=5 offset=801 table_len=32 fill=62.5% (5000075 attempts)
// square 60: bits=5 offset=833 table_len=32 fill=62.5% (5000048 attempts)
// square 61: bits=5 offset=861 table_len=32 fill=56.2% (5000141 attempts)
// square 62: bits=5 offset=892 table_len=31 fill=48.4% (5002632 attempts)
// square  9: bits=5 offset=919 table_len=32 fill=53.1% (5000844 attempts)
// square 10: bits=5 offset=949 table_len=32 fill=56.2% (5000568 attempts)
// square 11: bits=5 offset=981 table_len=32 fill=62.5% (5000629 attempts)
// square 12: bits=5 offset=1012 table_len=32 fill=62.5% (5000418 attempts)
// square 13: bits=5 offset=1038 table_len=32 fill=56.2% (5001427 attempts)
// square 14: bits=5 offset=1069 table_len=32 fill=53.1% (5002794 attempts)
// square 17: bits=5 offset=1097 table_len=30 fill=60.0% (5001039 attempts)
// square 18: bits=7 offset=1125 table_len=128 fill=56.2% (5001352 attempts)
// square 19: bits=7 offset=1251 table_len=128 fill=62.5% (5011607 attempts)
// square 20: bits=7 offset=1379 table_len=128 fill=75.0% (5022305 attempts)
// square 21: bits=7 offset=1500 table_len=127 fill=63.0% (5004896 attempts)
// square 22: bits=5 offset=1627 table_len=32 fill=56.2% (5003401 attempts)
// square 25: bits=5 offset=1658 table_len=32 fill=62.5% (5003560 attempts)
// square 26: bits=7 offset=1689 table_len=128 fill=75.0% (5000722 attempts)
// square 27: bits=9 offset=1804 table_len=512 fill=75.0% (5000362 attempts)
// square 28: bits=9 offset=2316 table_len=512 fill=100.0% (5066804 attempts)
// square 29: bits=7 offset=2828 table_len=128 fill=75.0% (5001699 attempts)
// square 30: bits=5 offset=2955 table_len=32 fill=62.5% (5000006 attempts)
// square 33: bits=5 offset=2987 table_len=32 fill=62.5% (5000677 attempts)
// square 34: bits=7 offset=3018 table_len=128 fill=62.5% (5001377 attempts)
// square 35: bits=9 offset=3146 table_len=512 fill=87.5% (5022503 attempts)
// square 36: bits=9 offset=3658 table_len=512 fill=100.0% (5074263 attempts)
// square 37: bits=7 offset=4170 table_len=128 fill=75.0% (5001078 attempts)
// square 38: bits=5 offset=4298 table_len=32 fill=62.5% (5000971 attempts)
// square 41: bits=5 offset=4328 table_len=32 fill=56.2% (5000607 attempts)
// square 42: bits=7 offset=4360 table_len=128 fill=56.2% (5005856 attempts)
// square 43: bits=7 offset=4486 table_len=128 fill=62.5% (5001934 attempts)
// square 44: bits=7 offset=4610 table_len=128 fill=62.5% (5003319 attempts)
// square 45: bits=7 offset=4731 table_len=128 fill=56.2% (5000341 attempts)
// square 46: bits=5 offset=4855 table_len=30 fill=60.0% (5000265 attempts)
// square 49: bits=5 offset=4884 table_len=32 fill=46.9% (5000025 attempts)
// square 50: bits=5 offset=4914 table_len=31 fill=58.1% (5000691 attempts)
// square 51: bits=5 offset=4945 table_len=32 fill=65.6% (5000927 attempts)
// square 52: bits=5 offset=4977 table_len=31 fill=64.5% (5000070 attempts)
// [thread 9] seed 16986587351727726250: table size 39.71 Kb (in 169.01s)
// square 53: bits=5 offset=5007 table_len=31 fill=51.6% (5000287 attempts)
// [thread 10] seed 9424380621860533918: table size 39.80 Kb (in 169.60s)
// [thread 5] seed 1035986090179171509: table size 39.45 Kb (in 169.63s)
// [thread 6] seed 15881013158765235087: table size 39.89 Kb (in 170.31s)
// square 54: bits=5 offset=5030 table_len=32 fill=53.1% (5001250 attempts)
// [thread 0] seed 15512407338438524295: table size 39.55 Kb (in 170.39s)
// [thread 1] seed 15348192573181382059: table size 39.91 Kb (in 170.61s)
// [thread 11] seed 1781602018382584721: table size 39.66 Kb (in 170.62s)
// [thread 4] seed 12408660808952000385: table size 39.70 Kb (in 170.99s)
// [thread 7] seed 8885303498275828532: table size 39.86 Kb (in 171.03s)
// [thread 2] seed 4036907382994161593: table size 39.58 Kb (in 171.17s)
// [thread 8] seed 9351298888654082800: table size 39.63 Kb (in 172.58s)
// [thread 3] seed 8865639049760002203: table size 39.82 Kb (in 172.73s)
// Best bishop global table: 39.45 Kb out of 12 threads (total time 172.74s)
pub const BISHOP_MAGICS: [MagicEntry; 64] = [
    MagicEntry { number: 4623024251195253769, blocker_mask: 0x0040201008040200, inverse_blocker_mask: 0xffbfdfeff7fbfdff, offset: 0, index_bits: 6, shift: 58 },
    MagicEntry { number: 2473059215997666304, blocker_mask: 0x0000402010080400, inverse_blocker_mask: 0xffffbfdfeff7fbff, offset: 231, index_bits: 5, shift: 59 },
    MagicEntry { number: 4543459206103048, blocker_mask: 0x0000004020100a00, inverse_blocker_mask: 0xffffffbfdfeff5ff, offset: 259, index_bits: 5, shift: 59 },
    MagicEntry { number: 6346205537395540000, blocker_mask: 0x0000000040221400, inverse_blocker_mask: 0xffffffffbfddebff, offset: 289, index_bits: 5, shift: 59 },
    MagicEntry { number: 1199573628520960, blocker_mask: 0x0000000002442800, inverse_blocker_mask: 0xfffffffffdbbd7ff, offset: 320, index_bits: 5, shift: 59 },
    MagicEntry { number: 594653411308224544, blocker_mask: 0x0000000204085000, inverse_blocker_mask: 0xfffffffdfbf7afff, offset: 347, index_bits: 5, shift: 59 },
    MagicEntry { number: 4614667928472657924, blocker_mask: 0x0000020408102000, inverse_blocker_mask: 0xfffffdfbf7efdfff, offset: 377, index_bits: 5, shift: 59 },
    MagicEntry { number: 3684017101630177296, blocker_mask: 0x0002040810204000, inverse_blocker_mask: 0xfffdfbf7efdfbfff, offset: 55, index_bits: 6, shift: 58 },
    MagicEntry { number: 1164189885226600456, blocker_mask: 0x0020100804020000, inverse_blocker_mask: 0xffdfeff7fbfdffff, offset: 403, index_bits: 5, shift: 59 },
    MagicEntry { number: 288354703134466052, blocker_mask: 0x0040201008040000, inverse_blocker_mask: 0xffbfdfeff7fbffff, offset: 917, index_bits: 5, shift: 59 },
    MagicEntry { number: 22535676501166336, blocker_mask: 0x00004020100a0000, inverse_blocker_mask: 0xffffbfdfeff5ffff, offset: 949, index_bits: 5, shift: 59 },
    MagicEntry { number: 2295919328888080, blocker_mask: 0x0000004022140000, inverse_blocker_mask: 0xffffffbfddebffff, offset: 979, index_bits: 5, shift: 59 },
    MagicEntry { number: 578995704295587973, blocker_mask: 0x0000000244280000, inverse_blocker_mask: 0xfffffffdbbd7ffff, offset: 1011, index_bits: 5, shift: 59 },
    MagicEntry { number: 576786905148990466, blocker_mask: 0x0000020408500000, inverse_blocker_mask: 0xfffffdfbf7afffff, offset: 1037, index_bits: 5, shift: 59 },
    MagicEntry { number: 584254554144, blocker_mask: 0x0002040810200000, inverse_blocker_mask: 0xfffdfbf7efdfffff, offset: 1068, index_bits: 5, shift: 59 },
    MagicEntry { number: 972777875008913477, blocker_mask: 0x0004081020400000, inverse_blocker_mask: 0xfffbf7efdfbfffff, offset: 432, index_bits: 5, shift: 59 },
    MagicEntry { number: 4652219248570085393, blocker_mask: 0x0010080402000200, inverse_blocker_mask: 0xffeff7fbfdfffdff, offset: 450, index_bits: 5, shift: 59 },
    MagicEntry { number: 1160841392227360784, blocker_mask: 0x0020100804000400, inverse_blocker_mask: 0xffdfeff7fbfffbff, offset: 1091, index_bits: 5, shift: 59 },
    MagicEntry { number: 4503677005022209, blocker_mask: 0x004020100a000a00, inverse_blocker_mask: 0xffbfdfeff5fff5ff, offset: 1122, index_bits: 7, shift: 57 },
    MagicEntry { number: 1172062112329499140, blocker_mask: 0x0000402214001400, inverse_blocker_mask: 0xffffbfddebffebff, offset: 1249, index_bits: 7, shift: 57 },
    MagicEntry { number: 19422358049652864, blocker_mask: 0x0000024428002800, inverse_blocker_mask: 0xfffffdbbd7ffd7ff, offset: 1377, index_bits: 7, shift: 57 },
    MagicEntry { number: 4683884350895292512, blocker_mask: 0x0002040850005000, inverse_blocker_mask: 0xfffdfbf7afffafff, offset: 1499, index_bits: 7, shift: 57 },
    MagicEntry { number: 3377708480954434, blocker_mask: 0x0004081020002000, inverse_blocker_mask: 0xfffbf7efdfffdfff, offset: 1624, index_bits: 5, shift: 59 },
    MagicEntry { number: 6926820502796320800, blocker_mask: 0x0008102040004000, inverse_blocker_mask: 0xfff7efdfbfffbfff, offset: 482, index_bits: 5, shift: 59 },
    MagicEntry { number: 6865359295643712, blocker_mask: 0x0008040200020400, inverse_blocker_mask: 0xfff7fbfdfffdfbff, offset: 506, index_bits: 5, shift: 59 },
    MagicEntry { number: 74555754334142498, blocker_mask: 0x0010080400040800, inverse_blocker_mask: 0xffeff7fbfffbf7ff, offset: 1656, index_bits: 5, shift: 59 },
    MagicEntry { number: 19142510393165830, blocker_mask: 0x0020100a000a1000, inverse_blocker_mask: 0xffdfeff5fff5efff, offset: 1688, index_bits: 7, shift: 57 },
    MagicEntry { number: 13835093310588387488, blocker_mask: 0x0040221400142200, inverse_blocker_mask: 0xffbfddebffebddff, offset: 1787, index_bits: 9, shift: 55 },
    MagicEntry { number: 144260327914151940, blocker_mask: 0x0002442800284400, inverse_blocker_mask: 0xfffdbbd7ffd7bbff, offset: 2299, index_bits: 9, shift: 55 },
    MagicEntry { number: 18582296333385792, blocker_mask: 0x0004085000500800, inverse_blocker_mask: 0xfffbf7afffaff7ff, offset: 2810, index_bits: 7, shift: 57 },
    MagicEntry { number: 2252917857075266, blocker_mask: 0x0008102000201000, inverse_blocker_mask: 0xfff7efdfffdfefff, offset: 2938, index_bits: 5, shift: 59 },
    MagicEntry { number: 9260669670333222946, blocker_mask: 0x0010204000402000, inverse_blocker_mask: 0xffefdfbfffbfdfff, offset: 537, index_bits: 5, shift: 59 },
    MagicEntry { number: 576606459071046660, blocker_mask: 0x0004020002040800, inverse_blocker_mask: 0xfffbfdfffdfbf7ff, offset: 569, index_bits: 5, shift: 59 },
    MagicEntry { number: 576557793870876675, blocker_mask: 0x0008040004081000, inverse_blocker_mask: 0xfff7fbfffbf7efff, offset: 2970, index_bits: 5, shift: 59 },
    MagicEntry { number: 36033367133652098, blocker_mask: 0x00100a000a102000, inverse_blocker_mask: 0xffeff5fff5efdfff, offset: 2999, index_bits: 7, shift: 57 },
    MagicEntry { number: 4899934021678793728, blocker_mask: 0x0022140014224000, inverse_blocker_mask: 0xffddebffebddbfff, offset: 3127, index_bits: 9, shift: 55 },
    MagicEntry { number: 288270044469674240, blocker_mask: 0x0044280028440200, inverse_blocker_mask: 0xffbbd7ffd7bbfdff, offset: 3639, index_bits: 9, shift: 55 },
    MagicEntry { number: 54060929448968321, blocker_mask: 0x0008500050080400, inverse_blocker_mask: 0xfff7afffaff7fbff, offset: 4150, index_bits: 7, shift: 57 },
    MagicEntry { number: 79441057415680, blocker_mask: 0x0010200020100800, inverse_blocker_mask: 0xffefdfffdfeff7ff, offset: 4277, index_bits: 5, shift: 59 },
    MagicEntry { number: 38077712073730, blocker_mask: 0x0020400040201000, inverse_blocker_mask: 0xffdfbfffbfdfefff, offset: 599, index_bits: 5, shift: 59 },
    MagicEntry { number: 153160081042579456, blocker_mask: 0x0002000204081000, inverse_blocker_mask: 0xfffdfffdfbf7efff, offset: 631, index_bits: 5, shift: 59 },
    MagicEntry { number: 288237042749449232, blocker_mask: 0x0004000408102000, inverse_blocker_mask: 0xfffbfffbf7efdfff, offset: 4308, index_bits: 5, shift: 59 },
    MagicEntry { number: 18016546665399296, blocker_mask: 0x000a000a10204000, inverse_blocker_mask: 0xfff5fff5efdfbfff, offset: 4338, index_bits: 7, shift: 57 },
    MagicEntry { number: 30804109953024, blocker_mask: 0x0014001422400000, inverse_blocker_mask: 0xffebffebddbfffff, offset: 4459, index_bits: 7, shift: 57 },
    MagicEntry { number: 9946200331774995456, blocker_mask: 0x0028002844020000, inverse_blocker_mask: 0xffd7ffd7bbfdffff, offset: 4586, index_bits: 7, shift: 57 },
    MagicEntry { number: 4683778848446747904, blocker_mask: 0x0050005008040200, inverse_blocker_mask: 0xffafffaff7fbfdff, offset: 4714, index_bits: 7, shift: 57 },
    MagicEntry { number: 36997452308864, blocker_mask: 0x0020002010080400, inverse_blocker_mask: 0xffdfffdfeff7fbff, offset: 4836, index_bits: 5, shift: 59 },
    MagicEntry { number: 9079837365109248, blocker_mask: 0x0040004020100800, inverse_blocker_mask: 0xffbfffbfdfeff7ff, offset: 660, index_bits: 5, shift: 59 },
    MagicEntry { number: 450362731047036968, blocker_mask: 0x0000020408102000, inverse_blocker_mask: 0xfffffdfbf7efdfff, offset: 682, index_bits: 5, shift: 59 },
    MagicEntry { number: 5044032003664060416, blocker_mask: 0x0000040810204000, inverse_blocker_mask: 0xfffffbf7efdfbfff, offset: 4868, index_bits: 5, shift: 59 },
    MagicEntry { number: 288934068025296896, blocker_mask: 0x00000a1020400000, inverse_blocker_mask: 0xfffff5efdfbfffff, offset: 4899, index_bits: 5, shift: 59 },
    MagicEntry { number: 1154364682691413060, blocker_mask: 0x0000142240000000, inverse_blocker_mask: 0xffffebddbfffffff, offset: 4931, index_bits: 5, shift: 59 },
    MagicEntry { number: 2324984411487224321, blocker_mask: 0x0000284402000000, inverse_blocker_mask: 0xffffd7bbfdffffff, offset: 4961, index_bits: 5, shift: 59 },
    MagicEntry { number: 6756929795825985, blocker_mask: 0x0000500804020000, inverse_blocker_mask: 0xffffaff7fbfdffff, offset: 4986, index_bits: 5, shift: 59 },
    MagicEntry { number: 9511971857538220097, blocker_mask: 0x0000201008040200, inverse_blocker_mask: 0xffffdfeff7fbfdff, offset: 5017, index_bits: 5, shift: 59 },
    MagicEntry { number: 72602163905060, blocker_mask: 0x0000402010080400, inverse_blocker_mask: 0xffffbfdfeff7fbff, offset: 709, index_bits: 5, shift: 59 },
    MagicEntry { number: 144117389259452740, blocker_mask: 0x0002040810204000, inverse_blocker_mask: 0xfffdfbf7efdfbfff, offset: 105, index_bits: 6, shift: 58 },
    MagicEntry { number: 432345573396586752, blocker_mask: 0x0004081020400000, inverse_blocker_mask: 0xfffbf7efdfbfffff, offset: 740, index_bits: 5, shift: 59 },
    MagicEntry { number: 1135795663030281, blocker_mask: 0x000a102040000000, inverse_blocker_mask: 0xfff5efdfbfffffff, offset: 771, index_bits: 5, shift: 59 },
    MagicEntry { number: 2305851805307506864, blocker_mask: 0x0014224000000000, inverse_blocker_mask: 0xffebddbfffffffff, offset: 803, index_bits: 5, shift: 59 },
    MagicEntry { number: 36028865757380928, blocker_mask: 0x0028440200000000, inverse_blocker_mask: 0xffd7bbfdffffffff, offset: 835, index_bits: 5, shift: 59 },
    MagicEntry { number: 13835058606150521184, blocker_mask: 0x0050080402000000, inverse_blocker_mask: 0xffaff7fbfdffffff, offset: 863, index_bits: 5, shift: 59 },
    MagicEntry { number: 9994698458861825, blocker_mask: 0x0020100804020000, inverse_blocker_mask: 0xffdfeff7fbfdffff, offset: 886, index_bits: 5, shift: 59 },
    MagicEntry { number: 90635097190049832, blocker_mask: 0x0040201008040200, inverse_blocker_mask: 0xffbfdfeff7fbfdff, offset: 167, index_bits: 6, shift: 58 },
];
