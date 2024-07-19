#![feature(portable_simd)]
use std::cmp::min;
use std::simd::{prelude::*, LaneCount, SupportedLaneCount};

pub fn scalar_hamming(x: &[u8], y: &[u8]) -> usize {
    x.iter().zip(y).filter(|(a, b)| a != b).count()
}

// Courtesy ScottMCM
pub fn scalar_hamming1b(a: &[u8], b: &[u8]) -> u32 {
    std::iter::zip(a, b).map(|(a, b)| a != b).fold(0, |a, b| a + b as u32)
}

pub fn simd_chunk_xor_hd<const N: usize>(x: &[u8], y: &[u8]) -> usize
where
    LaneCount<N>: SupportedLaneCount, {
    let mut differences: usize = 0;
    let ones: Simd<u8, N> = Simd::splat(1);

    let mut x = x.chunks_exact(N * 255);
    let mut y = y.chunks_exact(N * 255);

    for (c1, c2) in x.by_ref().zip(y.by_ref()) {
        let mut accum: Simd<u8, N> = Simd::splat(0);

        let mut c1 = c1.chunks_exact(N);
        let mut c2 = c2.chunks_exact(N);

        for (v1, v2) in c1.by_ref().zip(c2.by_ref()) {
            let v1: Simd<u8, N> = Simd::from_slice(v1);
            let v2: Simd<u8, N> = Simd::from_slice(v2);
            accum += ones.simd_min(v1 ^ v2);
        }

        let accum2: Simd<u16, N> = accum.cast();
        differences += accum2.reduce_sum() as usize;
    }

    let x = x.remainder();
    let y = y.remainder();
    let mut accum: Simd<u8, N> = Simd::splat(0);
    let mut c1 = x.chunks_exact(N);
    let mut c2 = y.chunks_exact(N);

    for (v1, v2) in c1.by_ref().zip(c2.by_ref()) {
        let v1: Simd<u8, N> = Simd::from_slice(v1);
        let v2: Simd<u8, N> = Simd::from_slice(v2);
        accum += ones.simd_min(v1 ^ v2);
    }
    let accum2: Simd<u16, N> = accum.cast();
    differences += accum2.reduce_sum() as usize;

    let r1 = c1.remainder();
    let r2 = c2.remainder();
    differences += r1.iter().zip(r2).filter(|(a, b)| a != b).count();
    return differences;
}

// Needs more cowbell, err, functional style
pub fn simd_fold_ne_hd<const N: usize>(x: &[u8], y: &[u8]) -> usize
where
    LaneCount<N>: SupportedLaneCount, {
    let mut differences: usize = 0;

    let mut x = x.chunks_exact(N * 255);
    let mut y = y.chunks_exact(N * 255);

    for (c1, c2) in x.by_ref().zip(y.by_ref()) {
        let c1 = c1.chunks_exact(N);
        let c2 = c2.chunks_exact(N);
        differences += std::iter::zip(c1, c2)
            .map(|(s1, s2)| Simd::from_slice(s1).simd_ne(Simd::from_slice(s2)).to_int().cast::<u8>())
            .fold(Simd::splat(0), |a, b| a - b)
            .cast::<u16>()
            .reduce_sum() as usize;
    }

    let x = x.remainder();
    let y = y.remainder();
    let mut c1 = x.chunks_exact(N);
    let mut c2 = y.chunks_exact(N);

    differences += std::iter::zip(c1.by_ref(), c2.by_ref())
        .map(|(s1, s2)| Simd::from_slice(s1).simd_ne(Simd::from_slice(s2)).to_int().cast::<u8>())
        .fold(Simd::splat(0), |a, b| a - b)
        .cast::<u16>()
        .reduce_sum() as usize;

    let r1 = c1.remainder();
    let r2 = c2.remainder();
    differences += r1.iter().zip(r2).filter(|(a, b)| a != b).count() as usize;
    return differences;
}

pub fn simd_chunk_select_hd<const N: usize>(x: &[u8], y: &[u8]) -> usize
where
    LaneCount<N>: SupportedLaneCount, {
    let mut differences: usize = 0;
    let ones: Simd<u8, N> = Simd::splat(1);
    let zeros: Simd<u8, N> = Simd::splat(0);

    let mut x = x.chunks_exact(N * 255);
    let mut y = y.chunks_exact(N * 255);

    for (c1, c2) in x.by_ref().zip(y.by_ref()) {
        let mut accum: Simd<u8, N> = zeros;

        let mut c1 = c1.chunks_exact(N);
        let mut c2 = c2.chunks_exact(N);

        for (v1, v2) in c1.by_ref().zip(c2.by_ref()) {
            let v1: Simd<u8, N> = Simd::from_slice(v1);
            let v2: Simd<u8, N> = Simd::from_slice(v2);
            let m = v1.simd_ne(v2);
            accum += m.select(ones, zeros);
        }

        let accum2: Simd<u16, N> = accum.cast();
        differences += accum2.reduce_sum() as usize;
    }

    let x = x.remainder();
    let y = y.remainder();
    let mut accum: Simd<u8, N> = zeros;
    let mut c1 = x.chunks_exact(N);
    let mut c2 = y.chunks_exact(N);

    for (v1, v2) in c1.by_ref().zip(c2.by_ref()) {
        let v1: Simd<u8, N> = Simd::from_slice(v1);
        let v2: Simd<u8, N> = Simd::from_slice(v2);
        let m = v1.simd_ne(v2);
        accum += m.select(ones, zeros);
    }
    let accum2: Simd<u16, N> = accum.cast();
    differences += accum2.reduce_sum() as usize;

    let r1 = c1.remainder();
    let r2 = c2.remainder();
    differences += r1.iter().zip(r2).filter(|(a, b)| a != b).count();
    return differences;
}

// Switching to u32 return type doesn't help
pub fn simd_chunk_ne_hd<const N: usize>(x: &[u8], y: &[u8]) -> usize
where
    LaneCount<N>: SupportedLaneCount, {
    let mut differences: usize = 0;

    let mut x = x.chunks_exact(N * 255);
    let mut y = y.chunks_exact(N * 255);

    for (c1, c2) in x.by_ref().zip(y.by_ref()) {
        let mut accum: Simd<u8, N> = Simd::splat(0);

        let mut c1 = c1.chunks_exact(N);
        let mut c2 = c2.chunks_exact(N);

        for (v1, v2) in c1.by_ref().zip(c2.by_ref()) {
            let v1: Simd<u8, N> = Simd::from_slice(v1);
            let v2: Simd<u8, N> = Simd::from_slice(v2);

            // True => -1, so - -1 => +1
            accum -= v1.simd_ne(v2).to_int().cast::<u8>();
        }

        differences += accum.cast::<u16>().reduce_sum() as usize;
    }

    let x = x.remainder();
    let y = y.remainder();
    let mut accum: Simd<u8, N> = Simd::splat(0);
    let mut c1 = x.chunks_exact(N);
    let mut c2 = y.chunks_exact(N);

    for (v1, v2) in c1.by_ref().zip(c2.by_ref()) {
        let v1: Simd<u8, N> = Simd::from_slice(v1);
        let v2: Simd<u8, N> = Simd::from_slice(v2);
        // True => -1, so - -1 => +1
        accum -= v1.simd_ne(v2).to_int().cast::<u8>();
    }
    differences += accum.cast::<u16>().reduce_sum() as usize;

    let r1 = c1.remainder();
    let r2 = c2.remainder();
    differences += r1.iter().zip(r2).filter(|(a, b)| a != b).count() as usize;
    return differences;
}

pub fn simd_reduce_ne_hd<const N: usize>(x: &[u8], y: &[u8]) -> usize
where
    LaneCount<N>: SupportedLaneCount, {
    let mut differences: usize = 0;

    let mut x = x.chunks_exact(N);
    let mut y = y.chunks_exact(N);

    for (c1, c2) in x.by_ref().zip(y.by_ref()) {
        let v1: Simd<u8, N> = Simd::from_slice(c1);
        let v2: Simd<u8, N> = Simd::from_slice(c2);
        let m = v1.simd_ne(v2).to_int();
        // True => -1, so - -1 => +1
        differences += (-m.reduce_sum()) as usize;
    }

    let r1 = x.remainder();
    let r2 = y.remainder();
    differences += r1.iter().zip(r2).filter(|(a, b)| a != b).count();

    return differences;
}

pub fn simd_chunk_eq_hd<const N: usize>(x: &[u8], y: &[u8]) -> usize
where
    LaneCount<N>: SupportedLaneCount, {
    let mut matches: usize = 0;
    let limit = min(x.len(), y.len());

    let mut x = x.chunks_exact(N * 255);
    let mut y = y.chunks_exact(N * 255);

    for (c1, c2) in x.by_ref().zip(y.by_ref()) {
        let mut accum: Simd<u8, N> = Simd::splat(0);

        let mut c1 = c1.chunks_exact(N);
        let mut c2 = c2.chunks_exact(N);

        for (v1, v2) in c1.by_ref().zip(c2.by_ref()) {
            let v1: Simd<u8, N> = Simd::from_slice(v1);
            let v2: Simd<u8, N> = Simd::from_slice(v2);
            let m = v1.simd_eq(v2).to_int();
            // True => -1, so - -1 => +1
            accum -= m.cast();
            //println!("{accum:?}");
        }

        let accum2: Simd<u16, N> = accum.cast();
        matches += accum2.reduce_sum() as usize;
    }

    let x = x.remainder();
    let y = y.remainder();
    let mut accum: Simd<u8, N> = Simd::splat(0);
    let mut c1 = x.chunks_exact(N);
    let mut c2 = y.chunks_exact(N);

    for (v1, v2) in c1.by_ref().zip(c2.by_ref()) {
        let v1: Simd<u8, N> = Simd::from_slice(v1);
        let v2: Simd<u8, N> = Simd::from_slice(v2);
        let m = v1.simd_eq(v2).to_int();
        // True => -1, so - -1 => +1
        accum -= m.cast();
        //println!("{accum:?}");
    }
    let accum2: Simd<u16, N> = accum.cast();
    matches += accum2.reduce_sum() as usize;

    let r1 = c1.remainder();
    let r2 = c2.remainder();
    matches += r1.iter().zip(r2).filter(|(a, b)| a == b).count();
    return limit - matches;
}

// Inspired by "triple_accel" by Daniel Liu (and portions elsewhere)
pub fn simd_for_ne_hd<const N: usize>(x: &[u8], y: &[u8]) -> usize
where
    LaneCount<N>: SupportedLaneCount, {
    let limit = min(x.len(), y.len());
    let refresh_len = limit / (255 * N);
    let mut differences: usize = 0;

    for i in 0..refresh_len {
        let mut accum: Simd<u8, N> = Simd::splat(0);
        for j in (i * 255)..((i + 1) * 255) {
            let v1: Simd<u8, N> = Simd::from_slice(&x[j * N..]);
            let v2: Simd<u8, N> = Simd::from_slice(&y[j * N..]);
            let m = v1.simd_ne(v2).to_int();
            // True => -1, so - -1 => +1
            accum -= m.cast();
        }
        let accum2: Simd<u16, N> = accum.cast();
        differences += accum2.reduce_sum() as usize;
    }

    let word_len = limit / N;
    let mut accum: Simd<u8, N> = Simd::splat(0);
    for i in (refresh_len * 255)..word_len {
        let v1: Simd<u8, N> = Simd::from_slice(&x[i * N..]);
        let v2: Simd<u8, N> = Simd::from_slice(&y[i * N..]);
        let m = v1.simd_ne(v2).to_int();
        // True => -1, so - -1 => +1
        accum -= m.cast();
    }
    let accum2: Simd<u16, N> = accum.cast();
    differences += accum2.reduce_sum() as usize;

    for i in word_len * N..limit {
        if x[i] != y[i] {
            differences += 1;
        }
    }

    return differences;
}

pub fn simd_while_ne_hd<const N: usize>(x: &[u8], y: &[u8]) -> usize
where
    LaneCount<N>: SupportedLaneCount, {
    let limit = min(x.len(), y.len());
    let mut differences: usize = 0;

    let mut p = 0;
    let mut accum: Simd<u8, N> = Simd::splat(0);
    while p < (limit - N) {
        if p % (256 * N) == 0 {
            let accum2: Simd<u16, N> = accum.cast();
            differences += accum2.reduce_sum() as usize;
            accum = Simd::splat(0);
        }

        let v1: Simd<u8, N> = Simd::from_slice(&x[p..]);
        let v2: Simd<u8, N> = Simd::from_slice(&y[p..]);
        let m = v1.simd_ne(v2).to_int();
        // True => -1, so - -1 => +1
        accum -= m.cast();
        p += N;
    }

    let accum2: Simd<u16, N> = accum.cast();
    differences += accum2.reduce_sum() as usize;

    for i in p..limit {
        if x[i] != y[i] {
            differences += 1;
        }
    }
    return differences;
}

// Requires both vectors to have the same alignment
pub fn simd_aligned_ne_hd<const N: usize>(x: &[u8], y: &[u8]) -> usize
where
    LaneCount<N>: SupportedLaneCount, {
    let (p1, m1, s1) = x.as_simd::<N>();
    let (p2, m2, s2) = y.as_simd::<N>();

    if p1.len() != p2.len() {
        return simd_chunk_ne_hd(x, y);
    }
    let mut m1 = m1.chunks_exact(255);
    let mut m2 = m2.chunks_exact(255);
    let mut differences: usize = 0;

    for (c1, c2) in m1.by_ref().zip(m2.by_ref()) {
        let mut accum: Simd<u8, N> = Simd::splat(0);

        for (v1, v2) in c1.iter().zip(c2) {
            let m = v1.simd_ne(*v2).to_int();
            // True => -1, so - -1 => +1
            accum -= m.cast();
        }
        let accum2: Simd<u16, N> = accum.cast();
        differences += accum2.reduce_sum() as usize;
    }

    let c1 = m1.remainder();
    let c2 = m2.remainder();
    let mut accum: Simd<u8, N> = Simd::splat(0);

    for (v1, v2) in c1.iter().zip(c2) {
        let m = v1.simd_ne(*v2).to_int();
        // True => -1, so - -1 => +1
        accum -= m.cast();
    }
    let accum2: Simd<u16, N> = accum.cast();
    differences += accum2.reduce_sum() as usize;
    differences += p1.iter().zip(p2.iter()).filter(|(a, b)| a != b).count();
    differences += s1.iter().zip(s2.iter()).filter(|(a, b)| a != b).count();

    return differences;
}

// Requires both vectors to have the same alignment
pub fn simd_aligned_eq_hd<const N: usize>(x: &[u8], y: &[u8]) -> usize
where
    LaneCount<N>: SupportedLaneCount, {
    let limit = min(x.len(), y.len());
    let mut matches: usize = 0;

    let (p1, m1, s1) = x.as_simd::<N>();
    let (p2, m2, s2) = y.as_simd::<N>();

    if p1.len() != p2.len() {
        return simd_chunk_eq_hd(x, y);
    }

    let mut m1 = m1.chunks_exact(255);
    let mut m2 = m2.chunks_exact(255);

    for (c1, c2) in m1.by_ref().zip(m2.by_ref()) {
        let mut accum: Simd<u8, N> = Simd::splat(0);

        for (v1, v2) in c1.iter().zip(c2) {
            let m = v1.simd_eq(*v2).to_int();
            // True => -1, so - -1 => +1
            accum -= m.cast();
        }
        let accum2: Simd<u16, N> = accum.cast();
        matches += accum2.reduce_sum() as usize;
    }

    let c1 = m1.remainder();
    let c2 = m2.remainder();
    let mut accum: Simd<u8, N> = Simd::splat(0);

    for (v1, v2) in c1.iter().zip(c2) {
        let m = v1.simd_eq(*v2).to_int();
        // True => -1, so - -1 => +1
        accum -= m.cast();
    }
    let accum2: Simd<u16, N> = accum.cast();
    matches += accum2.reduce_sum() as usize;
    matches += p1.iter().zip(p2.iter()).filter(|(a, b)| a == b).count();
    matches += s1.iter().zip(s2.iter()).filter(|(a, b)| a == b).count();

    return limit - matches;
}

pub static S1: &[u8; 1610] = b"upjwfrjrksuuhjcpwvsgpvgskpfednbqeculnqqjqfndahaawdtfacjvorajqpswolceuglvspjlbguehblvupikgptirrjkbgagwcsefubtafjrngahqsolpujbdocbeapkfggfmiarfuhrghuknidshdqqlposnhavdoqfpsnrqlrdghcnggruvnssufaafeepcbcdtwukntdajhhrcvphwgcconsfreakifbmnnifrukrmtrlgcijhetjowibsrtgvbpmmauqidarjfoauowhmsolitlfjavpudjubkrsfwtslodilaiklbkqpdfhleirakqbeebihufdmfglducdjuvnhlcqdmqvtwwmkdenfairjagmctktafpkwteifeuasdkchwibhjrnqperdiqlqqvpilwoofuranehdtomfanbwguioqkjkkvfobtabwvieanvbapcwkmeqtialwuehsntqrjabefsbkaquickhwtfhndcsjmosjickelnfjwnshhvreufqncjlngvuosbqmdmdwwcaunbegtkanosrqmwvpjkbuswhioqcoeussogrkuidjnrwajcaqefsuddikgbogfueleilsjfegfbbnlucidoqgdultnnwqueugsjwrebngnvploiwepevaowkmoqvelrwibbftepgagdqmswjagqcajlcpnsurcsujogvtornfalvmseftluefklnbprvbakqlrbtpsnmfefwnlclbndhelluhulmskeivpjjaegkwmtcwevbcustdksvfqcroiioodwiltkbwerdgapfhlkneevbfjjmusjmuscmubbowrraqdgviussngiopashnvqoitohjtchikonrgquwgkpnurkvipnegnvtpudkaiogvbhlwjibuiiptkegpbdtvqookrmgorbpkqsoscudugtgpgnqqutedoclwmhdmasqbdiiojgjkscuwslockjrjafhhaihrgolknqjdehrveqspuebijmweqtkrtgksawactdbtqdkqcsslmvcegenohchmfbhjoegmoieljigcbnbnecfbwbwsdoguiijcvoqeomrfhwslqfofinqnhelrngkapovgolwodtjukgffrteojsuajavhdetarjsnwaprsqudpbjbsjkratsfrgfptjhbwretspvrbdwtjdmqbvqlocggsnvktfvlniwbaqjfjvpdcfuwmofsrjmckptacvkwcsiaumhqirpmhpftkcrsuhjbqhvwwttspbsdumegtkgiofkfuqnjmdvidfuqnfsbkmdsbthajdooadnosawmbamjvchqnopadgtrckvjbwukmlenifgbdopkjwjjfjipipohtbdtvphwwsgrjtdbgcbfwaavijksgernnsrmctgouwgfhfkdhocsjutlfleuwtidvepksooesktwfjlbiwkdlcjpmduffpcdwaddmwjvplmriasrvwfnewkdeckwvrpsnutqmrcnndkjouatpvnhgwvlcauhajbusfkjhopplwlctkvliodqckqndcwmttgjbbfbjoohojvjsjbddufqbkpotrwldubmdig";
pub static S2: &[u8; 1610] = b"upQwfrjrEsCuhjcpwDsMpvgHkpfednPqeculnqqjIfndDhaaOdMfacjvoraMSpswoIceTVKGsIRlbguShbFvuLikgpIOTrLRbgagNcsefuGVafJrAMTNRsolpujVdHAbePpkfKWfmiarfuErgBFknOdshdqqlpGNnhavdPqfpsHrqlrdWScnggrJvnssufaafeeWcCPdtwDknKdaPhhrcNphwgCcoOsWPeakiFbQnnifrukrmtrBgcijhQLPoOibsrtgvbpNmauMiTaQjfoauoAhmJoCitPfjavpuPjRbQrsfwtslodUlaBklbGqAdfhleiQakqWeSRihuIdDHgldAVdjuvnEMcqdmqvAwwBkNenfairjagmctUPafpDwteUfeWasdkchwiMhBrSqMerHiHNqKvpilwBofQranDhdELmfanbwEuioqDjkkvBobIEbwOieanFbapcLJmeqMialwuKhPntqrjabefsbkaqVickhwtfJndPUJmPOjiBkLNnfAwLIhhvEeufqncjlngvuosbDmdUdJwcaunbIgtkanAsrqmwvpjPbRWNBioqcoeBssogrCuidjnrGaPHaqefsuddOkgbogfAelUiQsJfegIbADPuIBdoqgduGtnnAquOSgIjwrebngnvploiwepevJowkmoWGIlMwibbftApgRgdFmswjRIqBaIlIMnsWPcsujEgvtornfaTvTDeftNuefklnbpLvbaMqFrbtpsnmfefWnlNlbndhFlluhulmTkeivVjEaeLkwmtcwevbcustFJsvfqNroiiooHwLltkTwerdTapJhlknKevbfjjmusjmuScmGTbowEraqdgviussngiopashnvToitohjtchikonrVquwOSpnIrHvJFPegnvtpuWkaiogvVIlwjiCuiipUkegpMdtvToMkrmgoBbpkqNoscudugAWpJnTKAtedoclwVhdNNsqGdiioOgjkscALsloPkjDjafChTihrgolJnqjdehrveqspuMGNjmweTDkrIgksOwactdbJqdkqcKslmvcePFnIhADmfbhKoegmoieljiPcbnEVEcfbwDwsdoguAiMcvoqeomrfhwslqfNfinqnhelQnBkapGvgolwodtjukgffrNJojHuaLaIhdetarjsnUUprsqudpbEPsjDratsfKLfpOOhWwUEWsGIQbEwMjdmGJvqQoFgOsnOktWvlnUGbOqjfjvpdPfuwmofsrCmckptacvkwcsiaumhqiRpQIRftkcrsBhjBqhBwPtMNpbsdWmFLtkWiofkfuFnSmdvidOuqnfCbkmPPbthajdooIdnosawmOamjvchqnGpadgtrAkvjbwukmlenHfgbBopkjKjjTjiUipohtbdtvphFwKgrjPdbNcWVwaBFijksgerGJsrSCGgouwgfhfkQhocGjutlfleuwtKMvepksooesVtwfTlbiwkdlcjpOduffpTdwaVdmwjvplmrMasrvVfnGwkDeckHvrpsSuPVmrcNnCTjouAtpvnhgwUlcQuhajQusfkjhoFplSlctkvlKodEckqnRcwmStgjbbfbjoohoTEjsPSdVWfLEkpTtrwldubmdiW";

pub static L1: &[u8; 9303] = b"fhgmetghuhdaetaolqamlslwdaudnfeocikakucaawgqiwslrhnadawturriirvevidrqnucbdkkdrmqgpqsorbedriwpmedudjuvjockoulieolcdlosjttewnfwtpuhhwtecfqautoatalnvnfmvgbakbkojaatrojtcfnfivifrjwuoqiqgoreoqfaskevgrwprrvkluhmjclkpjnfpvriogccgeftguqpvsnlneqpfkmavendqpmvlqqqfltmaivttgqtibsueimsrcsvsmgkkpjqsnqlcuctcvbvhmcovvdwkjhpvihcgnarqfgofgkfnrvhwuusbwnhwmwkevpsjvmoecqfvicqebljhvqgthigdnqahdmjijlnoqnvokrivldasuibcrbfgskuwigqavkdpkqokoogbrualunvpalavrwmklpkunhqevaioflmrisngilhsowomgglomoagpqvjhcosenlfctaoumqgewnniconeahuiamalebeqqkfqkbftdukwrtfpuplhfhslifjpopnphoiqhgjmctcektjapkeankahtasvmsqjijswlphindaodctltopcreoagjuloheducejfdmhsrgskkwlobulngdiudaooolwpprcbjbhebvqdlunfruckvawftqkwswiwhmshaieowmvwhbwvlncssmuqdhjfqdggtrpahsitpleqftatqtgwdosdrlgsgbotvopnsgcsomnqivkrlvknqufcdvhgloqwwaugibosgiddusickpbbebsjsdtvsipifprpbcrtaqwattslbqvpiefhgeoebaujhfanmefapasbgpsnnpstffjpgpaccmrqvrrovutoktcuiqirobnrqdfaplvmpwejwhbgvdajkblmflwndkahtlnoedhswpiubsotrricjgecoelkfhdqhmcjvmrrriewfsmklmmhagwnbgvgiaftcrslefgfssmtdudikedwrhsweuapcmipbeuhspihavohlasepbdqfdvomlujtjsgisabvjmlcteqrwnhrlsonsbtjtucidovrqfcbmnwqhefhciilboavlsmakwfvhiwtuuqibovttwbalqqwkqlsushwggiarsibekdseblimfpsrlfrgtrhnuqcqsqjhahauidwftbijavtpgfuikjlelkgnsbhhfrtdnbquwhnkwmckcvpbrulhojvmdtivdiofksidghbhiktogtudilwgeeinbldmdslwvartjpgmnweuakpooturclqqfifempphhwupdiblbiwcutgwldsblgubgiumfcpilggrqwtvwicqfkepksvnthbweohifptfkrbaomhehjdeivcslfeqceihlrjmjkjbewtqljaulhemodhepqbmvtkcksichtwpiufvjjrjuufjjbalemmsniwlokhhchcusgwidomccgnvufwogebegvtltfjnlpvwdwvtbcsoutddqoesvripctbkeljhfqebhmoiqvjqmoecnjucvigqednjndavbgsnrvtngacppdkaobddscharwmnrbnkehqqubfbrqfgwbpknbncjtqldntntmofwtdsdmmvmqnswaatdkbagbrpugtfffpsioiqjrrhmqtbtgqkrsogjgodtvmwagbhptnltnhuvgokirurkpqewhwsvbrdbmubjnpmagdqqfbntqscqweklueqlaqgcmqbhtpirvatfmpmcqmibmuswbjebmadnhdmjflpegbcvubmjntcfptdgikekhqphoibciagevmbkdtqtjiktdkrsnnvpvocunwbpdpdlarwtckrbkgqionvwaodwvcdunqbjbkbbggpfvafvdgqbveokqmdofuhuckrjfcdqgeqiqdjilnpetvlbbmscojpjmcibdvfnpumttapvqsvkiqiufgeuoevkmnhtouhpdmlcpbswqngrtsnehhgiilwvderfcbevhkldrdledbcsintkswbqwoowbocohqrtouudhiwwrjafaahndmwuutfudheimipicqegpbuthnhwpowrrkrknfrcewafjqrcavibkrvgiodeeivklfikkgipermiinjjkmltdjcwblglnrouijkgcdjspdiabkiitjgeddqqvloilakmemwmhvjbghlwhgumesscemrbawmedpmpldgragakvbuejvnvrkcwjlevkmkkkuuhqjlkriugujaompqrslfwunoqrdslbgmfgdaqwbdpkmfcbgqvbbssjvrrlhujknkebkldwdabrvmhmostpreivmknpoijiswbamhnokgorcmwkhltdiiwoejpmpioefhrshvttimwsvctfedmcsbsvwgmsfefgpmqocvmokhjdthkrqikfkubocwsptfspvitkdfwdpfdebbfpcqbuhkhalqdgssetgvnwfmmwjudhupdstddpwqerosskffpgedfdfjsuinbukljilnvuuhpaerijpmnmemfplfwjadlosljqrjdattsgukhrboecrkvmesrwakqdwoikegabvljlfoiwuhvlnjgcucvdgtjwiikuidgqbkvqltmlocalnqsttkfculcmosarwlsllnsfduvlujphrippdwpeskfulerjthrutaagfmutkpcceabfbhpvcdncvbafbaudvvnwreqcrgnegpqhjrfejrrqrljnmvlheiqwaeekvsktcwkmhlvjqmbpdvfqcstaaawiwmcnvgjuujbeumigbjuenphnqlmaldmjskacewnipfhcqmtkdiwmfkrogjeterdeglwhfwjkawdrihspnfjgfrutvglkijsgdcemcgmjbadrbirtusnmfhekloqkcnvptfabwvkrlnqwwumphetcrgquihqnvpuwrdoorkucrtgkqcahlvtwwoaubjauihkjucdkkbipijqtluodnauffgsgqhetctrehnjmqvhnouuoukubnnacuggqlwhmskjevdebqnjfatibmbwrgvshfstfuugmmuqgrbbnflcsewfhvagfnvaufiqorgiqisjdsernhvaarpmdearufodtrmtpnecfdjgkjwdrasdrjlcdkboaaoorjcriiotcddljhsoksftbdehoubnpkkubaptnjplsnjnfttdfsjuptvlotopmtkfuopdedlusrjdmvvvgtqaohrvnnehdlbmhvwbltvienrpnorjdnwbtkeoatmcjhnpmrnkpbuvqntiquobbftsrrhoiamvwlaspkghgagsdnhlqigntvovehuewsdsjnpipmcghdllmbovucrmhmuaivsllivswardnialgdbahnrpmgjadoqprjsutprtfjuqavdhbcktweqbbefdmprkhsnhcwsvfpbaakfknplcjlksfrvtvdtuoettupumpwgobpbsmqgfgipjgpokvvfvevtgirfukigunmmwmipcbjowbbbrwguqwvcbbtgmedmnmjcfppwqmhdwhkplibatpilhegflhedugdpgkwsrijrllobmbansmqffrdlpftnobkujsrkjhnkoboeopukvteqcgiodkowoddbmtrtscggvjkjljidrbcwiaoomjtvsancnpfcmbtdnujlubtijegrwhflfeqnwewnooiebttlesndkcvgsitumvaeploapshlswesaetfkijkfqadahwqknfqpbpbkwpwhqkrnwhpgocjmaifrfaqvbgnquloiccfswpwjemkcgemjaiuqhjuroluaenfttkwnprcasbnpqdbragmwvbmsnsfcdfapqghwjndpfcpiqultncglhsfrfjgndpbpuobcvufiihwphjbkbehtsrhfhjmsamwggjihqdiqqptfngpqqkbfwecuqjlrtkimugfeakbrdnskknldcvnoahvqaaukkdanlcqfiifrdebbjdhohmiwthwplhpuuisdowivedprcvwlalgnofevshvdcgrldsdkptlvrvbflfombkkbooaoeradpipwclhrkuigwglobodlnwuavnjgrwgbjocswarjcbhorigqnnscdlnkmouorigvfdvbgqfewqvumiscddfgvgmulenglmpbnikoajqaflvnghahpjnllncjepwvltmddfdkdwkpgsisrglgrhewjugmjibgjmjnbjlktsojrwgpjcadkmnopjkckdifhrduuiiopuabucrowhgpnrjnbdrgiclhtbmacpkglmmhueoagotbiamsanctctlponknhtivpkskpbhrlqqurjbrmnvuqvafnsdmgjoknpvddtiasjgusqpuwghgtntkoewgkwjwiekthbdbwjtuqopogbctsvhdeabwaccctqnffdlkabsjbrmejhfjbmovbkjfwessokrenpvrkvwhrgbekofjlchvbpkimgmklqbrffobjnrfwjcagjnbdpatfdiplwgtbwcqjqeeapqidfqsjgnqewkhmedwsarblgkgugleqmiijuacpnbmukhtgfpeleeoooefovooogdgnssrbaisrfatdwvihusgfbbdkmgpnvbfmadqcalpjqogowsmescbjmvclprjblmbpagkwjbmvecjhqvcejaejlmqiqgfwqgckjiipvuwrcoeouqwvpsuliudbusdhavjnjkmcwhdwngsfkrikitmepkapneqncemhmlqdwuwlcposkmhlpfglvnuckumcqgpjqikofukipaptfhtumbdolnpdwcbisbeintpmnkbtgcdqjesbctrtwkauphrqepmfmbdusgdaptpiqjershhggbafuevmqkslslposuitnrnraiqaciknbnivaedtjsktwiasqsiqbksskcfwfsuadilqqhftmaoseejfguwrhuwsspohsmklgenuclwfpwrjetematijsfbswnskpfvghrdacfirgfsobjpwlwfsrfbclhqbeeogkuoaawkwtrorlpcerntiuqksijrqhfijvrpmroajbsdqephdhfofofnoakhwmnfdwqfnsqghtpoigkauwmdbhvujdqfuvkijrfmilschgsjoevucohqjipckbntpbemlbsopenvuhihhpnhedtqpadvwbrvovqdmshedrgaptkgkumamjjdjaldopftuqitvatmgsirrqngsvhioumbmfscjouarsrducuvgfwbotabsgdigavfowwibechshjiomnimapudfdafrtjlparbcpwiloanaicheclfojaekvltkdrcmwqrfrwrgakwvdcquivfvjofpeorwsvcvstvjigogsdbveukginhvbiuudpvwamgrikcwrshkmedjieeqjvorlcbrrjnmsasoredtvfonjnrtcaiwjtlqihdrwjdhicnvsvacrvrrqqshnvsjsekdmawklntkmesrioacwhhgfietjvuinpotmgorrthtpvgdiknioiimvkgaktkrdsgeepcvrqtmsfvcdqonoajbbgodbercpfsivpulcadnslbtawlglfvpftuwslmodnvbiledrwdstsrpodffecmcwetjablskgcnlqhkgcpsltdlqvnvjqjboeokimubuvhrajolhuemgjsremjnrvafwierwhgsdthjotjkvnbjcwtpnmtifalavqphpgpprtmdvtpuiafisofoptpabdofqfdgnetcnngdvvufvvipovceegnqlqnlruodbhqccwbdsqetdmfuhopmosllunrepkstgudjsufhufseldsjdspphhcjwdcwnbkoemwswjfiqqfvkfgvmweqiwedhgtimdqcdvvqbvkqvddpmaeeeaciwtclbfbdiilbkbifdfequcfrdjdwnmsutmhpmcsipdfmjambubdumnbbmiqbmdjvlvwkpnnedqnitaqcdpskffgmuqburosoauigrevcbwocmecdigorbgjnejqwknnwucobhqawbdpwgucubdwpruonfgtpvsukltkpiafpfqtagavvvkskangaptudpqohpbmqjjeflnjjpngocaeivnkqufatsfaaeammjgluufjmjcfpbbpmokwubowifjrsqnqqvtuopkitprmdmdfqewwwdepnmardldrqjocidpkhgauukthhlohovibfbkvhkrlcwejsfwetnvpdebioutuktjctucwskevqacqvpbdtshpfkhensvskfssfdlibhiwnegvcfjamhjeunaihlelrepuqccpsmfmwmwmvlnjgrsknpsehigluormrqdpwjmepssgojqjaucwbvqirrlageudbnrtvesnucdpeoinheafmqfpbejtdmvpguiamiudeiftungbusrupduihqhgwbsgmidbjbkgumgmvjjqkffpdjudahdtedikuhclspllukvafiekdehvkujjsfdbicnmsarvggqjpgmadlwvjwggretmbtvoodvsgkwulvjbouegsglafrpjkfltvfltpnjjrbbwpavctlmiauqwarachqskfwjtfrannnrpprkvftuktoqbmcnmrggsmfejjemmibenldrerqlnshtcbotvnpjbeqltgkscrngcikaqfojfthuouhpbpijbnqwuvkfpjlamelohjdmcspoimakldtjqjpmsrgfoclksqjgtjlfskjkvhbektaecafjfmlkdwpilbdpqdegpncdpeuumbrnkutqwcwujdstiwkqjrjrueatpjijbcoudclrmncddlqjqhsntraahpuesfrudcveaadtnsvjulnwigkrcstivoorwlbopvdbauoobcbncofmgcqfqqlmsdrmuegkljwmvrqhmquvfiwmldirlvwiqvmjgenketekqcvgbhmcudsfkutabbqrhtriuibmntpbwapgugevqvefcrhbkbpdwimglkmctibunwrmgsvwcetkcfwpworfdlrdrdhgmdhbqdicvdafaeihuueorrkdjgwnpncagaiqimowhlvnqjawpihksugawfpharmcwbhqpdiaawjheataqnboifaqvdqvrnabbotaaimcoipppvjfcokkemobbmttvgnjkrduckafoqntcmvpawkjfanmdcvfkplkpudpiwqaialjhdhdjmfalqnfaepkhwqipnhsslhagoqkgjkmebcmciuwcnmkniaefehdwdjhrojsgveowrbqgfkgokdkanhthuumijufdctrqatjedtndnbdgeiklhgvwodtsacbtgifiwmunopjeffnsqippbgusfbwnsdvmltpacnamfultsejrqfeuwpqpltniotbuhjqflsdklqjcvvhfenqhtjdkuwtwsvkcdavailequbpgupistlruuvghigkrruoqcfmlenrlwsekqiuoasktogqsltdurcmqppwgsrnhwrdnpeollnjcqtgkpullsmhwavgvkshvrbfsoakwmrpaiwdmwjskmrrdpaoeotjtgmwphretagwfhjhbvnlbekaoodrgnsgddkvooiwirwckkmuvrknfohjknhbwhsruqraeftwuanjufhmhraucrqgqcdsqqrntnnampghjcwoffipkdqangdeengkisspadjusphogckpjvivgubwlmtulcjlhlgqdqufnqpovpvrcsuhqaaasofpjfqobdabhjjamhviwfjribiruinrfwirqnnkvmluidvusaktsdojusnwnuolfqfodpjkrekjimadnimleuhdsjhgfghctscubbgpgmbnufscaqquwphrcuhgnutwmdrpbujmkpmtqijdipisrwahccqnemkkovntebhfbcogvhcedqgmvigmuvnrmhvdmdskikdjpohplctsldqekjiqdlcsasnhtwlfkuopuqkjgbtwgngbfofmwelakiesagnmbbjrhqntpjlbmmnwtqdoetwnkvwniispkmkdpqlomvvtlfenibhgfplknugrvpmeedawleosspglkfuafvcqqiwqwkdmfnqomvwkaaihfimrnavebibbcatoatuqqgmtrmatrsqrakdatrpduqupbtaaiklkapdwkkalijthomblbvibgtswqivadkrecqidkpdrphoudgjpuuuemtutivcpimjgsbgmplbuosmsewbgsobqdmeqtqwmncjettpguwjsfrfcenmwdmtocpmlsfbjsgjtvknthnquwdqrfepnhnbupmdwhffrjbvnbiqebiakwjtpebtkupqgriqchgpjropukfsgcltoucqfoiaesrhftkspqfudjwhbkptkvcnidugpuabajmmkvrkwbwbddtrossjgtsrjmlwfundmckvldfwwbtsptvkievsdkaieeekaeehecauvcuvdndfmnjpmelefomvkffdoohcfejuwueowfldnfkhmlqpfpncehmfhktieskrahmrgaiwiclajbnfuccntgpmjvtcvoaddhdudbgikogbndfsserrklwjpgrrirmmngichnevarmomclatvqrmeuqfiecilaqnoeuncqkivnujrocstqtvplaokauulkitmtaskjahurcftfonsnotqinawhtqlgabjhgdlrgrjivvfuidnvnifhtsenpqukswwpqtmnvudovllhqegqbfpugepmmwfrsualsscvnvufeiwgjuontlmifiiiutvkjdlnihuebhdtrfvrcdcbgamrwortkdtbvppubltvrfnfaavwqlwarsbeftoijocwdvfdwbfgmacdshucwkvluosgwhocjwtsnfojocghajqdqwuasrasacjvqjugblqmhnkutkmhqbcupdrhkvujhbvehbfcjgdnrndoknojcwwrijihuqkniovnuavglwsrmahrpieknmwhgkafvtsgekdwhmwtatpvwknmglsotfnqbtfejrmmwkfojfntitskfqjsmhejrphqegnpeojwcafmmildjtlrdvucrsqwedhjcucteeibprfqwwmcqndgubtvrpptmrraudttjcvmrglbsqooudesviupepcoisubtvqjpuqisvfuaohlswhbhsnfdfmihwgpvrjmoqsvajgjtddrallovjpndtdcmsrgglhajhtsshopwlmdsdafdvphjgpicoiokkqgidvqsseethigifukmlbmabujfdsrjlbdro";
pub static L2: &[u8; 9303] = b"GhgmetghuhdaetaolqRmlslwdaIdnKeocikakuEaawgqiwslrhnadawturriirvFvBdrPnuMbdkkdrmqgpqTorbedriHpmedudjuvjocRoulieoNcdlosjttewnRwUpuhhwtecfqauIoataLnvnLmvgbakbkojaatrojtcfnfivifrjwuOqiqgoreoTTaskevgrwprrvkluhmjclHpjnfpNrFogccgeftguqpvsnlneOpfkFavendqpmvlqqqfltmaivttgqtibsueUmsrHsvsmgkkpjqsnqlcuAtcvbvhmcoOvdKkjhBvihcLnarqfgofgkfnrHhwuusbwnhwmwEevKsjvmoecqfvicqebljhCqEtDiJdnqahdmBBjlnoqnvokriUldasuibFrbfgsOuwigqavkJpkqRkoogbrualJnBpalavrwmklpkunhqevaioflmrisnSiChsMDomgglomoagpqvjhKosOnlfctaoumqDewnnicVnNaTuiaBalebeqqkfqCbftdukwrtfpuplhfhslifjpopnphoiqhgjmcJcektjapkeaVkahtasvmsqjijswlphindaodctltoDcreoagjulJheHLcIjfdmhsGgDkkwlobRlngdiudaoAolwpprcbjbhebvqdlunfruckvaNftqkwswNwhmshaieowmvwhbwvlncssmuqdhjfqdggIrpahsitpleGftPtqtgwdosdrlgsgbotvopnsgcsomnqSvkrlvknqufMdvhgloqwwaugibosgiddusickpbbebsjsdtvsFpifprpbcrtaqwattslbqvpiefhgOoebaujhfanmefaRCsGgpsnnpOtffPWgpaccmrqvrKovutoktFWiqirobnrqdfapNvmpwejwhbGvdajkblmflwndkahtlnoedhswpiubOItrricjgecoelHfhdqhmTjMmrrriewfsmklmShagwnbDvgiafKcrslefgfssmtdudikedwrhsweuapcmipbeuPspihavohlasepbdqVdvomlujtUsgGsJbQPmlcteqrwUKrlsonsbCjtucidovrqfcbmnwqhefhciilboavCsmakwfvhiwtDuqibovtOwbTlqqwkqlsusBwggiWrsibekdseblimfpsrlfrgtrhnuqcqsqjhahaDTdwftbijavtpgfKikjlelkgnsbhhfrtdnbquwhnkwmckcvpbrulhojvAdtivdiofkQidThbhiktogEPdilWgeeinbldmdslwvaTtjVgmnHeuakpooturcKqqfifempphhwupdibRbiwcutgwldsblgubHiumfcpilggrqwCvBicKfkepksvnthbweohifpWfkrbaomhehjdeBvcFlfeqceihlrjmjkjbewtqljauAhemodhepNVmvUkIksVchtwpiufvjjrjuuSjDbalemmsniwFokhhchCusgwidoGccgnvuAwogebegvVltfjnlpvwdwvtbcsoHtddGoesvQipcJbkeljhFqebhmoiqvjqmoCcnjucvigqednjndavbgsnrvtngNcMpdkaobdCscharwmPrbnkehqqubfbrqfgJbpknbncjtqldKtnHOofwtdsdmmImqnsUaatdkbaMbrpugtBffpsioiqjPrhmqtbtgqkrGoPjgodtvmwagbhptnltnhuvgokirurkBqewhwsvbrdbmubjnNUagSqqfbnIqscqTeklueqlaFgcmqbhtpirAatfmpmcqmibmGswTjPbmadnhdmjflpegbcvubmjntcfptdgikekhqphoibciagevmbkdtqtjikIdkrsnnvpvLcunwbpdpdlarwtckrbkgqionvwaSdwAcdunqbjbkPbggpfvafvdgqbReokqmdofuhuckrjfcdqgeqiqLjilnpetvlDCmscojpjmcibdvfnpuOttapvqsvkKqiuSgeuoevkmnhtouJpdOlcpbswqngrtsnehKMiilwvderfcbevhkldrdledbcsintkswbqwOowbocohqLtouudhiwHrjafaahndGwuutfudheimipicqegpbuthnhwpowrrkrknfrFewafjqrcaBibkrvgiodeeivklfikkVipermiinjjkIltdjcRblglnrouiRkgcdjCpdiGbkiitjgedWqqMloilaSmFPwmhvjbHhGwWgumessceCrbaJmedpmQAdgragakvLuejvnvrNcwjlevkmkkkuuhqjlkriugujaomKqAslfwunTqrdslbgmfgdaUwHdpkmRcUgqvbWssjvrrlhujkKLebkldwIOTrvmhmosQpreFvmknpoijiswbamhnokgorcmwkhlKdiiwoejpmpioefhrsOvttiHWsvTtfeSmcsbsvwgGsfefgpmqocvmokhjdthkrqiCfkubVcwsptfspvitkdfGHpfdIbbfpcqbuhkhalqdgsseAgvnwfmmwjudhupdstddpBqerosskUfpgedfdfjsuinbukljilnvuuhpaerijOInmemfplTwjaWlosljqrjdatOsgukhrboecrkDmesrwakqdwoikMgabCljlfoiwuhvRnDgcMcvdgtGwiikuidgqbkvqltmlGcalnqsttkfcuLcmQsarwlsllnsfduvlujphrippdwpeskfulerjthrutaagfmutkpCceabfbhpvcdncvbaUbaudvvnwreScrgnegpqhjrfejrrqrljnmIlhQiqwaeekvsktcNkmhlLjqmbpdvfqcsSaaawiwmcnvgVuujbeumigbjuenphnqlmaKdmWskacewnipfhcqmtkdiwmfkrogjeterdeglwGfwjJawdrihspVfjgfrutvglkijsgdCemcgLjbadrbirFusnmfhekloqkcEvpGfabwvkrlnqwwumphJtcLgquihqnvpuwrdoorkucrtgkqBahlvtwwoaKAjauihkjucUkkbipijRFluodnauffgWgqhRtctrJhVjmqMhnouOoukubnnacuggqlwOmSkjevdebqFjfatibmbwrgvshfstfuCgmmuqgrbbnflcIewfhvagfnvAufiqorgiLisjdsMrnhvaarpmDearufodtrmApnecfdFgkjQdrasdrjlcdkbEaaooCjcLiiotcddljhsokAfLbdehoubnpkkubaptnjplsTjnfEtLfsjuptvlotoBmtNfuopdedlusrjdKvvvgtqaohrvnnehdlbmhvwbltvienrpnoFSdnwbtkeoatmUjhSpmOnkpbuvqntiquobbftsrrhoiamvwlaspkghgagsdnhlqignBvoFUhuewsEsjnpipmcghdllmbovucrmhNuaMvsllJvswardnialgdbahnrpmgjadoqprjCutprPfjuqavdhbcktwJqbbefdmprkhsnTcwVvfpSaakfknplcjlksfrvtvdtuoettNpumpwgoLpbsmqgfgipjgpokvvfvevtgirfukigunmmwmipcbjMwbbbrwguqwvcbbtgmedmnmjcCpQwqAhPwhkplibaVpilhegflhedugdpgHwsrijrlKJKmbansmEffrdlpftnobHujsUkjhnkoboeopuIvteqcgiUdkowoddbmtrtscggvjkjljidrbcwiaoomjtvsancWpfcmbtJLujlubtijegrwAflfeRnwewnoRieFttlesndkcUgsituHvaeploOpshRswesaeDfkijkfqQdahwqknfqTbpbkwpwhqkrnwhpgQcGVaifrfaqvbgnquloiccfswpwjRmkcgemjaiuqhjAroluaenfttkEnprcasbnpqdbragmwSbmsnsfcdfapqghwjBdpfcpiquTtncDlhsfrfjgndpbpOobcvuJiihwphjbkbehtsrOfhOmsamwHgEiCqdiqqItfngpqqkbfKKcuqjlrtkimugfeakbVdnskkJldcEnoahvqaaDkkdanlcqfiifrdebbjdhohmiwthwplApuuiFdowivedprcUwlaLgnofevshvdcgrldsdkptLvrvbfEfombkkbooaoeradpipHMlhrkuigwSlCbodlnwuavnjgrwgbjocOwarjcbhorigqnnscdlnkCouorigvKdvbgqfewqvWmiscddfgvgmulenglmpInikoajqMflvnghahpjVllncjeGwPltmddfdkdwkpHsisrgWgrhewjugmjibgjmUnbjlktsojrwgpjcBdkWJopjUcWdifhrduuiioPuabucrHwhgpnGjnbdEgiclhtbmacpkglmmhueoagotbiamGanRtctlponknJtivpkskpbLrlGqurNbrmnvuqvafnsdmgjoknpvdLtiasjVusqpuwgJgtUtkoeGgTwjwiekthbdbwjtuqNpogbctsvhdeabwacccUqnffdlkabOjbrmejhfjbmovbkjfwessokrenpvrkvwhrgbPkofIlchvbpkimgmklqbrffobjnDfwjcagGnbdTatfdiplwgtbwcqjqeeapqidfqsjgGqewkhmedwsarblgkCugleqmiiUuacpnbmukhtgfpeleeooPefovoooWdgnssrbaisrfatdwvihKsgfbbdkmgpnvbfmadJFalpjqogowsmeLcbjmvclprjblmbpagkNjbmvecjhqvcejaejlOqiqgfwqgckjiipvuwrFoeouqwHpsuliudbusdhavjnjkTcwhdwngsfkriKitmBpkapneqncemhmlCdwuwMTposkOhlpfglEnuckumcKgpjqikofukipaptEhtumbdoInpdwcbisbeintpmnkbtgcdqNesbctrtwkaKphrqepAImbdusgTaptpiqjerChhggbafOevmqkslsUposDitnKnraiqaciknbnIvaedtjsLtwiTsqsiqbksskJfwfsuadilDqhftmaoseejfgRwrhuwsspohsmPlBenuclwfpwrjetematijsfbswnskpfvghUdacfiMgfsMbjCwRwfsrfQclhqbeeoLkuUaawkwtroClpcRrntiPqksijrqhfijvEpmroVjbsdqepKdhfofofnoakhwmnfdwqfnEqghtpoVgkaDRCdbhvujdqfuvkijrfFilsBhgsjoevuBohqjipckbntpbemlbsoAenvuhihhpnheBtqpadvwbrvovqdmUheFrgaptkgkumamjjdjaWdopftuqitvatEgsJrDqngsvhioASbmfsHjouarsrduQuvgfwbotabsgdigavAowwibecWshjiomnimapudfdafGtjlparbcpwiloanaicheclfojaekvltkdLJmwqrQrwrgakwvdOquivMvjofpeorwJvcvstvjigogsdbveukgKnhvbiuuSpvwamgrikcwrshkmedDieLqjvorlcbBrjnmsaQorVdtQfoCjnrtcBiwjtlEihdrwjdhicnvsvacrvErQqChnvGSsekdmawDlntkmesrTLCcwhhgCietQvuinpoQmgorrthtpvgdiknioiimvkgaktkrdsgeeVcvrItmsfvcdqonoajbbgodbercpfBivpuAcadNslbtJwlglfvpftuwsKmodnvbileIrwdstFrEodVfEcmcBetjablskgcnlqhkgcpsltdlqvnvjTjboeokimubuvhIajolQuTmgjsremjnrvafwierwhgsdtUjotjkvnVjcwtpHmtifalavqpUCgpprtmdvtpuiafiNofoptpabdofqfdgnetcnnDdvvufvviIBvceegnqlqnlruodbhqccwbdsqetdmfuKopmosllOnrKpkstgREjsufhufseldsjdspIhhcjLdcwnbkoemwswjfiqJfvkfgvmweqiwedPgtiLdqcdVvqbvkqvddpmaeeeaciwtclbfbdiiVbkbiLdfequcfrdjGwnmsutmhpIcsiUdfPjambuBdumnbbmiqUmdjvlvwApJnedqnitaqcdpskffgmuqburosHauigrevcbwocmecdigorbgjnejqwknnwucHbhqLwIdpwgucubWApruonKgtpOsuklSkpiafSfqtagavvCkskangFptudpqoGpbmqjjeflnjjpngocaeivnJqufatsfaaeammjgluufWmAcfpbbpmokwubowVfjrsqnqDvtuopFitprmdMdfqewwwdepnmardldrqjocidpkQgauukKhhlVhovibKbkvhkrlcweJsfwetnvpdebioutuktjcGucwskevqEcqvpbdtshpfkhensvskfssfdlibhiwnegvcfjaKhjeunaihlBlrepuqPcpsmfmwFwmvlnjgrsknpsehigluUrmrqdpwjmepssgoTqjaMNwbvqirrlageudbnrtvesnWcdpeCinheafmqfpbejLdAvpguiaViudJiftuEgbusrupBuihqhgwbsgmidbjbkgumgmvjjqHffpdjudahdtedikuhclspllukvafiekdehHkVjjsPdbiQnmsarvggqjpgmCdlQvjRggHetmMtvoodvsgkwulTjboueIsQlafrpjPfltvfltpnjjrbbEHavctlmiauqwarachqskfwjtDrannnrpprkvftuktoqPmRVmrRgsmfejjemmibenldrIWqlnshtcbotvnpjbeqltgkscrnMcikaqfojfthuouhpbpijInqwuvkfpjlamelohjdmGspoimaklTSjqjNmsrgfoclJsqjgtjlfRkjkvhbektEecafjfmlkdVpilbdpqdHgpnAdpLuumbrnkutqwcwujdstiwkqjrjrueatpGijbcoudclrmncddlqjqDsntraahpueTfrudHveaadtnSvjulnwigkrcHtivoorwlbopvdbauoobcbncofmgcqfqqlmsdrOuegkljwmvrqhmquvIiwmldirPVwiqvmjgeQketekqcvgKhmcIdsfkAtFPbMrhtriuibmTtpbwapgugevLvefcrhbkHpdwimglkmctibuMwLmOsvwcetkcfwJworBdlrdrdhPmdhVqdicvdSfaeiOuueorrkdjgwnIncagaLqimowhlvnqjawpihHsugawfpharmcSbhqpdiaawjheataMnboiUaqvdqvrnabbotaaimcEipppvjfcokkemGbbmttvSnjkrduckafoqntcmvpawkjfanmIcvfkplkpudpiwqaialjhdhdAmfalqnfaepkhwqipnhsslhagoqkgjkmebcmciUwcnmkniaefehdwdjhrojsgveowrbqgfkgokdkaLhthuumijufdctrqatjedtndnQdgeiklhSvwoDtsacbtgifiwGunopjeffnsqippbgusfbwnsdvmltpacnTmfultsSjrqfeuwpqpltnSotbuhjqflsdRlqBcvvhfenqhtjdkuwtwHvkcWaLailequbpguFistNruuvghigkrruoqcfmlenrlwsekOiuoasktogqsltdurEmqppwgsrnhwrdnpeollnQJqtgkpullsmhwavgvkshvAbfsoakwmrpaDwdmwjsVmrrdpaoeotjtgmwKhretaMLfhjhbvnlbekaoodrgnsgUdkvooiwirwckkmVvrWnfohjknhbwhsruqraeKtKuaHjufhmhraucrqgqcLsqqrnWnnampJhjcwoffipkdqangdHenNkisspFdjEsphogckpjvTvguDwlmtElcjlhlgqdqufnqUovpvrcsuhqaaasofpjfqobdabhjjFmhJiwfjrPBiruinrfwirqTnkvmluDdvuVaktsJojusnwBuolfWfodpjkrekjimadnGmleuhdsMhgfgSctscubbgpgmFnufscAqquwphrcuhgnutwmdRQbujmkpmtqijdipisrwahccqnemkkovnteNhfbcogvBceSqgmvigmuvnrmhvdmdUkikdjpohpIcGsldqOkjiqdlcsasnhtwlfkuopuqkjFbtwgngbfofmwelKkiesPgnmbbjHhqDtpjlbmmnwtqdoetwnkvwniispkmkdpqloPvvtlMenibOHfplknugrvpmeedawleosspglkfuIfvcqqiPqwkdQfnqomvwCCaihfimrnavebibbcatoatuqqgmDrmatrsCrakdatrpKuqupbtaaiklkapdwkkSlijWhomblbvTbgtswqivadkrecqidkpdRHhoudgjpuuTemJuSivcpimjgsbgmplbuosPsewbgTobqdmeqtHwmRcjetLpguwjsfrfcenmwdmtocpmlsfbjsgjtvknthnquUdqrfepnNnbupmdwSffrjbvnBiqebiakwjtpebtkupqgriqchgpjropukfsgUltoucqfoiaesrhftkspqfudjwhbkptkvcnidugpuabajmmkvrkwbNbdWtrossjLtsrjmlwfundmckvldfwwbtsptvkievsdkaieeekaeeTecauvcuvdndfmnJpmelefoWvkfJdooBcfejuwueowfBdnfkhmlqpfpnceOmfhkSieLkraJmrgaiwiclajEnfuccntgpmMvtcvoaddhdAdSgMkogbndfsserrklwjpgrrirmmngichnevSrmomclatvqFmeuqViecilIqnoeLncqkivnujrocstqJvplaokauulkitmDaskjahOrcftfonsnoJqEnawhtqlgabjhgdlrgrjivNfuiLQvnifhtsenpqukswwpKtmnvudovllhqegqbfpugepmmwfrsualSMcvVvJfeDwgjuFntlmiCiKiutvkjdlnihuebRdtMAvrcdcbgPmrwortkdtbvpLubltvrfnfaavwqlwarRbeftoijocwdvfdwbfgmacQshucwkvluosgwhocjwtsnfojocghajqdqwuasrasacjvqjugblqmJnTutkmhSbcupdShPvujKbvehbfcjgdnrndoknojcwwrijihuqkniovnuavglwsrmahrpieknmwhgkafvtsgekdwhmwtatpvwknPglsotfnqJtfejrmmwkfSVfntitskfqjsmhejrphqegnpeojwcafmUildjtlrdvucrsqwJdhjcucteeibQFfqwwmcDndgIbtvrLptmrraudttjcvmrglbsqooudesviupepcoisubtvqjpuqisvfuaohMswhbhsnfdfmihwgVvrjmoqsvajgTtddrFllovjpndtdcmsrgglhajhtsshopwlmdsdafNvpOjgpicoiokkJgidvqsseethigifukmlbMabujfNsrjlbMrP";
