#![feature(portable_simd)]
use std::simd::*;
use std::cmp::min;

// Does auto-vectorize to some extent
#[inline(always)]
pub fn scalar_hamming(x: &[u8], y: &[u8]) -> usize {
    x.iter().zip(y).filter(|(a,b)| a != b ).count()
}

pub fn simd_chunk_xor_hd<const N: usize>( x: &[u8], y: &[u8]) -> usize
    where LaneCount<N>: SupportedLaneCount {

    let mut differences: usize = 0;
    let ones: Simd<u8,N> = Simd::splat(1);

    let mut x = x.chunks_exact(N * 255);
    let mut y = y.chunks_exact(N * 255);
    
    for (c1,c2) in x.by_ref().zip(y.by_ref()) {
        let mut accum: Simd<u8, N> = Simd::splat(0);

        let mut c1 = c1.chunks_exact( N );
        let mut c2 = c2.chunks_exact( N );

        for (v1, v2) in c1.by_ref().zip(c2.by_ref()) {
            let v1: Simd<u8,N> = Simd::from_slice(v1);
            let v2: Simd<u8,N> = Simd::from_slice(v2);
            accum += ones.min(v1 ^ v2);

            //println!("{accum:?}");    
        }
            
        let accum2: Simd<u16,N> = accum.cast();
        differences += accum2.reduce_sum() as usize;

    }
   
    let x = x.remainder();
    let y = y.remainder();
    let mut accum: Simd<u8, N> = Simd::splat(0);
    let mut c1 = x.chunks_exact( N );
    let mut c2 = y.chunks_exact( N );

    for (v1, v2) in c1.by_ref().zip(c2.by_ref()) {
        let v1: Simd<u8,N> = Simd::from_slice(v1);
        let v2: Simd<u8,N> = Simd::from_slice(v2);
        accum += ones.min(v1 ^ v2);
        
        //println!("{accum:?}");
    }
    let accum2: Simd<u16,N> = accum.cast();
    differences += accum2.reduce_sum() as usize;


    let r1 = c1.remainder();
    let r2 = c2.remainder();
    differences += r1.iter().zip(r2).filter( |(a,b)| a != b ).count();
    return differences;
}

pub fn simd_chunk_ne_hd<const N: usize>( x: &[u8], y: &[u8]) -> usize
    where LaneCount<N>: SupportedLaneCount {

    let mut differences: usize = 0;

    let mut x = x.chunks_exact(N * 255);
    let mut y = y.chunks_exact(N * 255);
    
    for (c1,c2) in x.by_ref().zip(y.by_ref()) {
        let mut accum: Simd<u8, N> = Simd::splat(0);

        let mut c1 = c1.chunks_exact( N );
        let mut c2 = c2.chunks_exact( N );

        for (v1, v2) in c1.by_ref().zip(c2.by_ref()) {
            let v1: Simd<u8,N> = Simd::from_slice(v1);
            let v2: Simd<u8,N> = Simd::from_slice(v2);
            let m = v1.lanes_ne(v2).to_int();
            // True => -1, so - -1 => +1
            accum -= m.cast();
            //println!("{accum:?}");
    
        }
            
        let accum2: Simd<u16,N> = accum.cast();
        differences += accum2.reduce_sum() as usize;

    }
   
    let x = x.remainder();
    let y = y.remainder();
    let mut accum: Simd<u8, N> = Simd::splat(0);
    let mut c1 = x.chunks_exact( N );
    let mut c2 = y.chunks_exact( N );

    for (v1, v2) in c1.by_ref().zip(c2.by_ref()) {
        let v1: Simd<u8,N> = Simd::from_slice(v1);
        let v2: Simd<u8,N> = Simd::from_slice(v2);
        let m = v1.lanes_ne(v2).to_int();
        // True => -1, so - -1 => +1
        accum -= m.cast();
        //println!("{accum:?}");

    }
    let accum2: Simd<u16,N> = accum.cast();
    differences += accum2.reduce_sum() as usize;


    let r1 = c1.remainder();
    let r2 = c2.remainder();
    differences += r1.iter().zip(r2).filter( |(a,b)| a != b ).count();
    return differences;
}

pub fn simd_chunk_eq_hd <const N: usize>( x: &[u8], y: &[u8]) -> usize
    where LaneCount<N>: SupportedLaneCount {
    //const N: usize = 16;

    let mut matches: usize = 0;
    let limit = min(x.len(),y.len());

    let mut x = x.chunks_exact(N * 255);
    let mut y = y.chunks_exact(N * 255);
    
    for (c1,c2) in x.by_ref().zip(y.by_ref()) {
        let mut accum: Simd<u8, N> = Simd::splat(0);

        let mut c1 = c1.chunks_exact( N );
        let mut c2 = c2.chunks_exact( N );

        for (v1, v2) in c1.by_ref().zip(c2.by_ref()) {
            let v1: Simd<u8,N> = Simd::from_slice(v1);
            let v2: Simd<u8,N> = Simd::from_slice(v2);
            let m = v1.lanes_eq(v2).to_int();
            // True => -1, so - -1 => +1
            accum -= m.cast();
            //println!("{accum:?}");
    
        }
            
        let accum2: Simd<u16,N> = accum.cast();
        matches += accum2.reduce_sum() as usize;

    }
   
    let x = x.remainder();
    let y = y.remainder();
    let mut accum: Simd<u8, N> = Simd::splat(0);
    let mut c1 = x.chunks_exact( N );
    let mut c2 = y.chunks_exact( N );

    for (v1, v2) in c1.by_ref().zip(c2.by_ref()) {
        let v1: Simd<u8,N> = Simd::from_slice(v1);
        let v2: Simd<u8,N> = Simd::from_slice(v2);
        let m = v1.lanes_eq(v2).to_int();
        // True => -1, so - -1 => +1
        accum -= m.cast();
        //println!("{accum:?}");

    }
    let accum2: Simd<u16,N> = accum.cast();
    matches += accum2.reduce_sum() as usize;


    let r1 = c1.remainder();
    let r2 = c2.remainder();
    matches += r1.iter().zip(r2).filter( |(a,b)| a == b ).count();
    return limit - matches;
}

pub fn simd_for_ne_hd<const N: usize>( x: &[u8], y: &[u8]) -> usize
    where LaneCount<N>: SupportedLaneCount {

    let limit = min(x.len(),y.len());
    let refresh_len = limit / (255 * N);
    let mut differences: usize = 0;

    for i in 0 .. refresh_len {
        let mut accum: Simd<u8, N> = Simd::splat(0);
        for j in (i*255)..((i+1)*255) {
            let v1: Simd<u8,N> = Simd::from_slice(&x[j*N..]);
            let v2: Simd<u8,N> = Simd::from_slice(&y[j*N..]);
            let m = v1.lanes_ne(v2).to_int();
            // True => -1, so - -1 => +1
            accum -= m.cast();
        }
        let accum2: Simd<u16,N> = accum.cast();
        differences += accum2.reduce_sum() as usize;
    }

    let word_len = limit >> 5;
    let mut accum: Simd<u8, N> = Simd::splat(0);
    for i in (refresh_len * 255)..word_len {
        let v1: Simd<u8,N> = Simd::from_slice(&x[i*N..]);
        let v2: Simd<u8,N> = Simd::from_slice(&y[i*N..]);
        let m = v1.lanes_ne(v2).to_int();
        // True => -1, so - -1 => +1
        accum -= m.cast();

    }
    let accum2: Simd<u16,N> = accum.cast();
    differences += accum2.reduce_sum() as usize;

    for i in word_len * N..limit {
        if x[i] != y[i] {
            differences += 1;
        }
    } 

    return differences;
}

pub fn simd_while_ne_hd <const N: usize>( x: &[u8], y: &[u8]) -> usize
    where LaneCount<N>: SupportedLaneCount {

    let limit = min(x.len(),y.len());
    let mut differences: usize = 0;

    let mut p = 0;
    let mut accum: Simd<u8, N> = Simd::splat(0);
    while p < (limit-N) {
        if p % (256 * N) == 0 {
            let accum2: Simd<u16,N> = accum.cast();
            differences += accum2.reduce_sum() as usize;
            accum = Simd::splat(0);

        }

        let v1: Simd<u8,N> = Simd::from_slice(&x[p..]);
        let v2: Simd<u8,N> = Simd::from_slice(&y[p..]);
        let m = v1.lanes_ne(v2).to_int();
        // True => -1, so - -1 => +1
        accum -= m.cast();
        p += N;
    }

    let accum2: Simd<u16,N> = accum.cast();
    differences += accum2.reduce_sum() as usize;

    for i in p..limit {
        if x[i] != y[i] {
            differences += 1;
        }
    } 
    return differences;
}


#[repr(C, align(64))]
pub struct AlignedVec(pub Vec<u8>);
impl AlignedVec {
    pub fn get_slice(&self) -> &[u8] {
        self.0.as_slice()
    }
}


pub fn simd_aligned_ne_hd<const N: usize>(x: &[u8], y: &[u8]) -> usize 
    where LaneCount<N>: SupportedLaneCount {

    #[allow(unused_variables)]
    let (p1, m1, s1) = x.as_simd::<N>();
    #[allow(unused_variables)]
    let (p2, m2, s2) = y.as_simd::<N>();


    let mut m1 = m1.chunks_exact(255);
    let mut m2 = m2.chunks_exact(255);
    let mut differences: usize = 0;
    
    for (c1,c2) in m1.by_ref().zip(m2.by_ref()) {
        let mut accum: Simd<u8, N> = Simd::splat(0);

        for (v1, v2) in c1.iter().zip(c2) {
            let m = v1.lanes_ne(*v2).to_int();
            // True => -1, so - -1 => +1
            accum -= m.cast();
            
        }
        let accum2: Simd<u16,N> = accum.cast();
        differences += accum2.reduce_sum() as usize;
    }

    let c1 = m1.remainder();
    let c2 = m2.remainder();
    let mut accum: Simd<u8, N> = Simd::splat(0);

    for (v1, v2) in c1.iter().zip(c2) {
        let m = v1.lanes_ne(*v2).to_int();
        // True => -1, so - -1 => +1
        accum -= m.cast();

    }
    let accum2: Simd<u16,N> = accum.cast();
    differences += accum2.reduce_sum() as usize;
    //let z = m1.iter().copied().zip(m2.iter().copied());
    differences += p1.iter().zip(p2.iter()).filter( |(a,b)| a != b ).count();
    differences += s1.iter().zip(s2.iter()).filter( |(a,b)| a != b ).count();

    return differences;
}

pub fn simd_aligned_eq_hd<const N: usize>(x: &[u8], y: &[u8]) -> usize 
    where LaneCount<N>: SupportedLaneCount {

    let limit = min(x.len(),y.len());
    let mut matches: usize = 0;

    #[allow(unused_variables)]
    let (p1, m1, s1) = x.as_simd::<N>();
    #[allow(unused_variables)]
    let (p2, m2, s2) = y.as_simd::<N>();


    let mut m1 = m1.chunks_exact(255);
    let mut m2 = m2.chunks_exact(255);
    
    for (c1,c2) in m1.by_ref().zip(m2.by_ref()) {
        let mut accum: Simd<u8, N> = Simd::splat(0);

        for (v1, v2) in c1.iter().zip(c2) {
            let m = v1.lanes_eq(*v2).to_int();
            // True => -1, so - -1 => +1
            accum -= m.cast();
            
        }
        let accum2: Simd<u16,N> = accum.cast();
        matches += accum2.reduce_sum() as usize;
    }

    let c1 = m1.remainder();
    let c2 = m2.remainder();
    let mut accum: Simd<u8, N> = Simd::splat(0);

    for (v1, v2) in c1.iter().zip(c2) {
        let m = v1.lanes_eq(*v2).to_int();
        // True => -1, so - -1 => +1
        accum -= m.cast();

    }
    let accum2: Simd<u16,N> = accum.cast();
    matches += accum2.reduce_sum() as usize;
    //let z = m1.iter().copied().zip(m2.iter().copied());
    matches += p1.iter().zip(p2.iter()).filter( |(a,b)| a == b ).count();
    matches += s1.iter().zip(s2.iter()).filter( |(a,b)| a == b ).count();

    return limit - matches;
}


pub static S1: &[u8; 1610] = b"upjwfrjrksuuhjcpwvsgpvgskpfednbqeculnqqjqfndahaawdtfacjvorajqpswolceuglvspjlbguehblvupikgptirrjkbgagwcsefubtafjrngahqsolpujbdocbeapkfggfmiarfuhrghuknidshdqqlposnhavdoqfpsnrqlrdghcnggruvnssufaafeepcbcdtwukntdajhhrcvphwgcconsfreakifbmnnifrukrmtrlgcijhetjowibsrtgvbpmmauqidarjfoauowhmsolitlfjavpudjubkrsfwtslodilaiklbkqpdfhleirakqbeebihufdmfglducdjuvnhlcqdmqvtwwmkdenfairjagmctktafpkwteifeuasdkchwibhjrnqperdiqlqqvpilwoofuranehdtomfanbwguioqkjkkvfobtabwvieanvbapcwkmeqtialwuehsntqrjabefsbkaquickhwtfhndcsjmosjickelnfjwnshhvreufqncjlngvuosbqmdmdwwcaunbegtkanosrqmwvpjkbuswhioqcoeussogrkuidjnrwajcaqefsuddikgbogfueleilsjfegfbbnlucidoqgdultnnwqueugsjwrebngnvploiwepevaowkmoqvelrwibbftepgagdqmswjagqcajlcpnsurcsujogvtornfalvmseftluefklnbprvbakqlrbtpsnmfefwnlclbndhelluhulmskeivpjjaegkwmtcwevbcustdksvfqcroiioodwiltkbwerdgapfhlkneevbfjjmusjmuscmubbowrraqdgviussngiopashnvqoitohjtchikonrgquwgkpnurkvipnegnvtpudkaiogvbhlwjibuiiptkegpbdtvqookrmgorbpkqsoscudugtgpgnqqutedoclwmhdmasqbdiiojgjkscuwslockjrjafhhaihrgolknqjdehrveqspuebijmweqtkrtgksawactdbtqdkqcsslmvcegenohchmfbhjoegmoieljigcbnbnecfbwbwsdoguiijcvoqeomrfhwslqfofinqnhelrngkapovgolwodtjukgffrteojsuajavhdetarjsnwaprsqudpbjbsjkratsfrgfptjhbwretspvrbdwtjdmqbvqlocggsnvktfvlniwbaqjfjvpdcfuwmofsrjmckptacvkwcsiaumhqirpmhpftkcrsuhjbqhvwwttspbsdumegtkgiofkfuqnjmdvidfuqnfsbkmdsbthajdooadnosawmbamjvchqnopadgtrckvjbwukmlenifgbdopkjwjjfjipipohtbdtvphwwsgrjtdbgcbfwaavijksgernnsrmctgouwgfhfkdhocsjutlfleuwtidvepksooesktwfjlbiwkdlcjpmduffpcdwaddmwjvplmriasrvwfnewkdeckwvrpsnutqmrcnndkjouatpvnhgwvlcauhajbusfkjhopplwlctkvliodqckqndcwmttgjbbfbjoohojvjsjbddufqbkpotrwldubmdig";
pub static S2: &[u8; 1610] = b"upQwfrjrEsCuhjcpwDsMpvgHkpfednPqeculnqqjIfndDhaaOdMfacjvoraMSpswoIceTVKGsIRlbguShbFvuLikgpIOTrLRbgagNcsefuGVafJrAMTNRsolpujVdHAbePpkfKWfmiarfuErgBFknOdshdqqlpGNnhavdPqfpsHrqlrdWScnggrJvnssufaafeeWcCPdtwDknKdaPhhrcNphwgCcoOsWPeakiFbQnnifrukrmtrBgcijhQLPoOibsrtgvbpNmauMiTaQjfoauoAhmJoCitPfjavpuPjRbQrsfwtslodUlaBklbGqAdfhleiQakqWeSRihuIdDHgldAVdjuvnEMcqdmqvAwwBkNenfairjagmctUPafpDwteUfeWasdkchwiMhBrSqMerHiHNqKvpilwBofQranDhdELmfanbwEuioqDjkkvBobIEbwOieanFbapcLJmeqMialwuKhPntqrjabefsbkaqVickhwtfJndPUJmPOjiBkLNnfAwLIhhvEeufqncjlngvuosbDmdUdJwcaunbIgtkanAsrqmwvpjPbRWNBioqcoeBssogrCuidjnrGaPHaqefsuddOkgbogfAelUiQsJfegIbADPuIBdoqgduGtnnAquOSgIjwrebngnvploiwepevJowkmoWGIlMwibbftApgRgdFmswjRIqBaIlIMnsWPcsujEgvtornfaTvTDeftNuefklnbpLvbaMqFrbtpsnmfefWnlNlbndhFlluhulmTkeivVjEaeLkwmtcwevbcustFJsvfqNroiiooHwLltkTwerdTapJhlknKevbfjjmusjmuScmGTbowEraqdgviussngiopashnvToitohjtchikonrVquwOSpnIrHvJFPegnvtpuWkaiogvVIlwjiCuiipUkegpMdtvToMkrmgoBbpkqNoscudugAWpJnTKAtedoclwVhdNNsqGdiioOgjkscALsloPkjDjafChTihrgolJnqjdehrveqspuMGNjmweTDkrIgksOwactdbJqdkqcKslmvcePFnIhADmfbhKoegmoieljiPcbnEVEcfbwDwsdoguAiMcvoqeomrfhwslqfNfinqnhelQnBkapGvgolwodtjukgffrNJojHuaLaIhdetarjsnUUprsqudpbEPsjDratsfKLfpOOhWwUEWsGIQbEwMjdmGJvqQoFgOsnOktWvlnUGbOqjfjvpdPfuwmofsrCmckptacvkwcsiaumhqiRpQIRftkcrsBhjBqhBwPtMNpbsdWmFLtkWiofkfuFnSmdvidOuqnfCbkmPPbthajdooIdnosawmOamjvchqnGpadgtrAkvjbwukmlenHfgbBopkjKjjTjiUipohtbdtvphFwKgrjPdbNcWVwaBFijksgerGJsrSCGgouwgfhfkQhocGjutlfleuwtKMvepksooesVtwfTlbiwkdlcjpOduffpTdwaVdmwjvplmrMasrvVfnGwkDeckHvrpsSuPVmrcNnCTjouAtpvnhgwUlcQuhajQusfkjhoFplSlctkvlKodEckqnRcwmStgjbbfbjoohoTEjsPSdVWfLEkpTtrwldubmdiW";

pub static L1: &[u8; 9303] = b"fhgmetghuhdaetaolqamlslwdaudnfeocikakucaawgqiwslrhnadawturriirvevidrqnucbdkkdrmqgpqsorbedriwpmedudjuvjockoulieolcdlosjttewnfwtpuhhwtecfqautoatalnvnfmvgbakbkojaatrojtcfnfivifrjwuoqiqgoreoqfaskevgrwprrvkluhmjclkpjnfpvriogccgeftguqpvsnlneqpfkmavendqpmvlqqqfltmaivttgqtibsueimsrcsvsmgkkpjqsnqlcuctcvbvhmcovvdwkjhpvihcgnarqfgofgkfnrvhwuusbwnhwmwkevpsjvmoecqfvicqebljhvqgthigdnqahdmjijlnoqnvokrivldasuibcrbfgskuwigqavkdpkqokoogbrualunvpalavrwmklpkunhqevaioflmrisngilhsowomgglomoagpqvjhcosenlfctaoumqgewnniconeahuiamalebeqqkfqkbftdukwrtfpuplhfhslifjpopnphoiqhgjmctcektjapkeankahtasvmsqjijswlphindaodctltopcreoagjuloheducejfdmhsrgskkwlobulngdiudaooolwpprcbjbhebvqdlunfruckvawftqkwswiwhmshaieowmvwhbwvlncssmuqdhjfqdggtrpahsitpleqftatqtgwdosdrlgsgbotvopnsgcsomnqivkrlvknqufcdvhgloqwwaugibosgiddusickpbbebsjsdtvsipifprpbcrtaqwattslbqvpiefhgeoebaujhfanmefapasbgpsnnpstffjpgpaccmrqvrrovutoktcuiqirobnrqdfaplvmpwejwhbgvdajkblmflwndkahtlnoedhswpiubsotrricjgecoelkfhdqhmcjvmrrriewfsmklmmhagwnbgvgiaftcrslefgfssmtdudikedwrhsweuapcmipbeuhspihavohlasepbdqfdvomlujtjsgisabvjmlcteqrwnhrlsonsbtjtucidovrqfcbmnwqhefhciilboavlsmakwfvhiwtuuqibovttwbalqqwkqlsushwggiarsibekdseblimfpsrlfrgtrhnuqcqsqjhahauidwftbijavtpgfuikjlelkgnsbhhfrtdnbquwhnkwmckcvpbrulhojvmdtivdiofksidghbhiktogtudilwgeeinbldmdslwvartjpgmnweuakpooturclqqfifempphhwupdiblbiwcutgwldsblgubgiumfcpilggrqwtvwicqfkepksvnthbweohifptfkrbaomhehjdeivcslfeqceihlrjmjkjbewtqljaulhemodhepqbmvtkcksichtwpiufvjjrjuufjjbalemmsniwlokhhchcusgwidomccgnvufwogebegvtltfjnlpvwdwvtbcsoutddqoesvripctbkeljhfqebhmoiqvjqmoecnjucvigqednjndavbgsnrvtngacppdkaobddscharwmnrbnkehqqubfbrqfgwbpknbncjtqldntntmofwtdsdmmvmqnswaatdkbagbrpugtfffpsioiqjrrhmqtbtgqkrsogjgodtvmwagbhptnltnhuvgokirurkpqewhwsvbrdbmubjnpmagdqqfbntqscqweklueqlaqgcmqbhtpirvatfmpmcqmibmuswbjebmadnhdmjflpegbcvubmjntcfptdgikekhqphoibciagevmbkdtqtjiktdkrsnnvpvocunwbpdpdlarwtckrbkgqionvwaodwvcdunqbjbkbbggpfvafvdgqbveokqmdofuhuckrjfcdqgeqiqdjilnpetvlbbmscojpjmcibdvfnpumttapvqsvkiqiufgeuoevkmnhtouhpdmlcpbswqngrtsnehhgiilwvderfcbevhkldrdledbcsintkswbqwoowbocohqrtouudhiwwrjafaahndmwuutfudheimipicqegpbuthnhwpowrrkrknfrcewafjqrcavibkrvgiodeeivklfikkgipermiinjjkmltdjcwblglnrouijkgcdjspdiabkiitjgeddqqvloilakmemwmhvjbghlwhgumesscemrbawmedpmpldgragakvbuejvnvrkcwjlevkmkkkuuhqjlkriugujaompqrslfwunoqrdslbgmfgdaqwbdpkmfcbgqvbbssjvrrlhujknkebkldwdabrvmhmostpreivmknpoijiswbamhnokgorcmwkhltdiiwoejpmpioefhrshvttimwsvctfedmcsbsvwgmsfefgpmqocvmokhjdthkrqikfkubocwsptfspvitkdfwdpfdebbfpcqbuhkhalqdgssetgvnwfmmwjudhupdstddpwqerosskffpgedfdfjsuinbukljilnvuuhpaerijpmnmemfplfwjadlosljqrjdattsgukhrboecrkvmesrwakqdwoikegabvljlfoiwuhvlnjgcucvdgtjwiikuidgqbkvqltmlocalnqsttkfculcmosarwlsllnsfduvlujphrippdwpeskfulerjthrutaagfmutkpcceabfbhpvcdncvbafbaudvvnwreqcrgnegpqhjrfejrrqrljnmvlheiqwaeekvsktcwkmhlvjqmbpdvfqcstaaawiwmcnvgjuujbeumigbjuenphnqlmaldmjskacewnipfhcqmtkdiwmfkrogjeterdeglwhfwjkawdrihspnfjgfrutvglkijsgdcemcgmjbadrbirtusnmfhekloqkcnvptfabwvkrlnqwwumphetcrgquihqnvpuwrdoorkucrtgkqcahlvtwwoaubjauihkjucdkkbipijqtluodnauffgsgqhetctrehnjmqvhnouuoukubnnacuggqlwhmskjevdebqnjfatibmbwrgvshfstfuugmmuqgrbbnflcsewfhvagfnvaufiqorgiqisjdsernhvaarpmdearufodtrmtpnecfdjgkjwdrasdrjlcdkboaaoorjcriiotcddljhsoksftbdehoubnpkkubaptnjplsnjnfttdfsjuptvlotopmtkfuopdedlusrjdmvvvgtqaohrvnnehdlbmhvwbltvienrpnorjdnwbtkeoatmcjhnpmrnkpbuvqntiquobbftsrrhoiamvwlaspkghgagsdnhlqigntvovehuewsdsjnpipmcghdllmbovucrmhmuaivsllivswardnialgdbahnrpmgjadoqprjsutprtfjuqavdhbcktweqbbefdmprkhsnhcwsvfpbaakfknplcjlksfrvtvdtuoettupumpwgobpbsmqgfgipjgpokvvfvevtgirfukigunmmwmipcbjowbbbrwguqwvcbbtgmedmnmjcfppwqmhdwhkplibatpilhegflhedugdpgkwsrijrllobmbansmqffrdlpftnobkujsrkjhnkoboeopukvteqcgiodkowoddbmtrtscggvjkjljidrbcwiaoomjtvsancnpfcmbtdnujlubtijegrwhflfeqnwewnooiebttlesndkcvgsitumvaeploapshlswesaetfkijkfqadahwqknfqpbpbkwpwhqkrnwhpgocjmaifrfaqvbgnquloiccfswpwjemkcgemjaiuqhjuroluaenfttkwnprcasbnpqdbragmwvbmsnsfcdfapqghwjndpfcpiqultncglhsfrfjgndpbpuobcvufiihwphjbkbehtsrhfhjmsamwggjihqdiqqptfngpqqkbfwecuqjlrtkimugfeakbrdnskknldcvnoahvqaaukkdanlcqfiifrdebbjdhohmiwthwplhpuuisdowivedprcvwlalgnofevshvdcgrldsdkptlvrvbflfombkkbooaoeradpipwclhrkuigwglobodlnwuavnjgrwgbjocswarjcbhorigqnnscdlnkmouorigvfdvbgqfewqvumiscddfgvgmulenglmpbnikoajqaflvnghahpjnllncjepwvltmddfdkdwkpgsisrglgrhewjugmjibgjmjnbjlktsojrwgpjcadkmnopjkckdifhrduuiiopuabucrowhgpnrjnbdrgiclhtbmacpkglmmhueoagotbiamsanctctlponknhtivpkskpbhrlqqurjbrmnvuqvafnsdmgjoknpvddtiasjgusqpuwghgtntkoewgkwjwiekthbdbwjtuqopogbctsvhdeabwaccctqnffdlkabsjbrmejhfjbmovbkjfwessokrenpvrkvwhrgbekofjlchvbpkimgmklqbrffobjnrfwjcagjnbdpatfdiplwgtbwcqjqeeapqidfqsjgnqewkhmedwsarblgkgugleqmiijuacpnbmukhtgfpeleeoooefovooogdgnssrbaisrfatdwvihusgfbbdkmgpnvbfmadqcalpjqogowsmescbjmvclprjblmbpagkwjbmvecjhqvcejaejlmqiqgfwqgckjiipvuwrcoeouqwvpsuliudbusdhavjnjkmcwhdwngsfkrikitmepkapneqncemhmlqdwuwlcposkmhlpfglvnuckumcqgpjqikofukipaptfhtumbdolnpdwcbisbeintpmnkbtgcdqjesbctrtwkauphrqepmfmbdusgdaptpiqjershhggbafuevmqkslslposuitnrnraiqaciknbnivaedtjsktwiasqsiqbksskcfwfsuadilqqhftmaoseejfguwrhuwsspohsmklgenuclwfpwrjetematijsfbswnskpfvghrdacfirgfsobjpwlwfsrfbclhqbeeogkuoaawkwtrorlpcerntiuqksijrqhfijvrpmroajbsdqephdhfofofnoakhwmnfdwqfnsqghtpoigkauwmdbhvujdqfuvkijrfmilschgsjoevucohqjipckbntpbemlbsopenvuhihhpnhedtqpadvwbrvovqdmshedrgaptkgkumamjjdjaldopftuqitvatmgsirrqngsvhioumbmfscjouarsrducuvgfwbotabsgdigavfowwibechshjiomnimapudfdafrtjlparbcpwiloanaicheclfojaekvltkdrcmwqrfrwrgakwvdcquivfvjofpeorwsvcvstvjigogsdbveukginhvbiuudpvwamgrikcwrshkmedjieeqjvorlcbrrjnmsasoredtvfonjnrtcaiwjtlqihdrwjdhicnvsvacrvrrqqshnvsjsekdmawklntkmesrioacwhhgfietjvuinpotmgorrthtpvgdiknioiimvkgaktkrdsgeepcvrqtmsfvcdqonoajbbgodbercpfsivpulcadnslbtawlglfvpftuwslmodnvbiledrwdstsrpodffecmcwetjablskgcnlqhkgcpsltdlqvnvjqjboeokimubuvhrajolhuemgjsremjnrvafwierwhgsdthjotjkvnbjcwtpnmtifalavqphpgpprtmdvtpuiafisofoptpabdofqfdgnetcnngdvvufvvipovceegnqlqnlruodbhqccwbdsqetdmfuhopmosllunrepkstgudjsufhufseldsjdspphhcjwdcwnbkoemwswjfiqqfvkfgvmweqiwedhgtimdqcdvvqbvkqvddpmaeeeaciwtclbfbdiilbkbifdfequcfrdjdwnmsutmhpmcsipdfmjambubdumnbbmiqbmdjvlvwkpnnedqnitaqcdpskffgmuqburosoauigrevcbwocmecdigorbgjnejqwknnwucobhqawbdpwgucubdwpruonfgtpvsukltkpiafpfqtagavvvkskangaptudpqohpbmqjjeflnjjpngocaeivnkqufatsfaaeammjgluufjmjcfpbbpmokwubowifjrsqnqqvtuopkitprmdmdfqewwwdepnmardldrqjocidpkhgauukthhlohovibfbkvhkrlcwejsfwetnvpdebioutuktjctucwskevqacqvpbdtshpfkhensvskfssfdlibhiwnegvcfjamhjeunaihlelrepuqccpsmfmwmwmvlnjgrsknpsehigluormrqdpwjmepssgojqjaucwbvqirrlageudbnrtvesnucdpeoinheafmqfpbejtdmvpguiamiudeiftungbusrupduihqhgwbsgmidbjbkgumgmvjjqkffpdjudahdtedikuhclspllukvafiekdehvkujjsfdbicnmsarvggqjpgmadlwvjwggretmbtvoodvsgkwulvjbouegsglafrpjkfltvfltpnjjrbbwpavctlmiauqwarachqskfwjtfrannnrpprkvftuktoqbmcnmrggsmfejjemmibenldrerqlnshtcbotvnpjbeqltgkscrngcikaqfojfthuouhpbpijbnqwuvkfpjlamelohjdmcspoimakldtjqjpmsrgfoclksqjgtjlfskjkvhbektaecafjfmlkdwpilbdpqdegpncdpeuumbrnkutqwcwujdstiwkqjrjrueatpjijbcoudclrmncddlqjqhsntraahpuesfrudcveaadtnsvjulnwigkrcstivoorwlbopvdbauoobcbncofmgcqfqqlmsdrmuegkljwmvrqhmquvfiwmldirlvwiqvmjgenketekqcvgbhmcudsfkutabbqrhtriuibmntpbwapgugevqvefcrhbkbpdwimglkmctibunwrmgsvwcetkcfwpworfdlrdrdhgmdhbqdicvdafaeihuueorrkdjgwnpncagaiqimowhlvnqjawpihksugawfpharmcwbhqpdiaawjheataqnboifaqvdqvrnabbotaaimcoipppvjfcokkemobbmttvgnjkrduckafoqntcmvpawkjfanmdcvfkplkpudpiwqaialjhdhdjmfalqnfaepkhwqipnhsslhagoqkgjkmebcmciuwcnmkniaefehdwdjhrojsgveowrbqgfkgokdkanhthuumijufdctrqatjedtndnbdgeiklhgvwodtsacbtgifiwmunopjeffnsqippbgusfbwnsdvmltpacnamfultsejrqfeuwpqpltniotbuhjqflsdklqjcvvhfenqhtjdkuwtwsvkcdavailequbpgupistlruuvghigkrruoqcfmlenrlwsekqiuoasktogqsltdurcmqppwgsrnhwrdnpeollnjcqtgkpullsmhwavgvkshvrbfsoakwmrpaiwdmwjskmrrdpaoeotjtgmwphretagwfhjhbvnlbekaoodrgnsgddkvooiwirwckkmuvrknfohjknhbwhsruqraeftwuanjufhmhraucrqgqcdsqqrntnnampghjcwoffipkdqangdeengkisspadjusphogckpjvivgubwlmtulcjlhlgqdqufnqpovpvrcsuhqaaasofpjfqobdabhjjamhviwfjribiruinrfwirqnnkvmluidvusaktsdojusnwnuolfqfodpjkrekjimadnimleuhdsjhgfghctscubbgpgmbnufscaqquwphrcuhgnutwmdrpbujmkpmtqijdipisrwahccqnemkkovntebhfbcogvhcedqgmvigmuvnrmhvdmdskikdjpohplctsldqekjiqdlcsasnhtwlfkuopuqkjgbtwgngbfofmwelakiesagnmbbjrhqntpjlbmmnwtqdoetwnkvwniispkmkdpqlomvvtlfenibhgfplknugrvpmeedawleosspglkfuafvcqqiwqwkdmfnqomvwkaaihfimrnavebibbcatoatuqqgmtrmatrsqrakdatrpduqupbtaaiklkapdwkkalijthomblbvibgtswqivadkrecqidkpdrphoudgjpuuuemtutivcpimjgsbgmplbuosmsewbgsobqdmeqtqwmncjettpguwjsfrfcenmwdmtocpmlsfbjsgjtvknthnquwdqrfepnhnbupmdwhffrjbvnbiqebiakwjtpebtkupqgriqchgpjropukfsgcltoucqfoiaesrhftkspqfudjwhbkptkvcnidugpuabajmmkvrkwbwbddtrossjgtsrjmlwfundmckvldfwwbtsptvkievsdkaieeekaeehecauvcuvdndfmnjpmelefomvkffdoohcfejuwueowfldnfkhmlqpfpncehmfhktieskrahmrgaiwiclajbnfuccntgpmjvtcvoaddhdudbgikogbndfsserrklwjpgrrirmmngichnevarmomclatvqrmeuqfiecilaqnoeuncqkivnujrocstqtvplaokauulkitmtaskjahurcftfonsnotqinawhtqlgabjhgdlrgrjivvfuidnvnifhtsenpqukswwpqtmnvudovllhqegqbfpugepmmwfrsualsscvnvufeiwgjuontlmifiiiutvkjdlnihuebhdtrfvrcdcbgamrwortkdtbvppubltvrfnfaavwqlwarsbeftoijocwdvfdwbfgmacdshucwkvluosgwhocjwtsnfojocghajqdqwuasrasacjvqjugblqmhnkutkmhqbcupdrhkvujhbvehbfcjgdnrndoknojcwwrijihuqkniovnuavglwsrmahrpieknmwhgkafvtsgekdwhmwtatpvwknmglsotfnqbtfejrmmwkfojfntitskfqjsmhejrphqegnpeojwcafmmildjtlrdvucrsqwedhjcucteeibprfqwwmcqndgubtvrpptmrraudttjcvmrglbsqooudesviupepcoisubtvqjpuqisvfuaohlswhbhsnfdfmihwgpvrjmoqsvajgjtddrallovjpndtdcmsrgglhajhtsshopwlmdsdafdvphjgpicoiokkqgidvqsseethigifukmlbmabujfdsrjlbdro";
pub static L2: &[u8; 9303] = b"fhgmetghuhdaetaoUqRmlslwdaIdnKeocikakuEaawgqiwslrhnadawturrJirvFvBdrPnuMbdkkdrmqgpqTorbedriHpmedudjuvjocRoulieoNcdlosjttewnRwUpuhhwtecfqauIoataLnvnLmvgbakbkojaatrojtcfnfivifrjwuOqiqgoreoTTaskevgrwprrvkluhmjclHpjnfpNrFogccgeftguqpvsnlneOpfkFavendqpmvlqqqfltmaivttgqtibsueUmsrHsvsmgkkpjqsnqlcuAtcvbvhmcoOvdKkjhBvihcLnarqfgofgkfnrHhwuusbwnhwmwEevKsjvmoecqfvicqebljhCqEtDiJdnqahdmBBjlnoqnvokriUldasuibFrbfgsOuwigqavkJpkqRkoogbrualJnBpalavrwmklpkunhqevaioflmrisnSiChsMDomgglomoagpqvjhKosOnlfctaoumqDewnnicVnNaTuiaBalebeqqkfqCbftdukwrtfpuplhfhslifjpopnphoiqhgjmcJcektjapkeaVkahtasvmsqjijswlphindaodctltoDcreoagjulJheHLcIjfdmhsGgDkkwlobRlngdiudaoAolwpprcbjbhebvqdlunfruckvaNftqkwswNwhmshaieowmvwhbwvlncssmuqdhjfqdggIrpahsitpleGftPtqtgwdosdrlgsgbotvopnsgcsomnqSvkrlvknqufMdvhgloqwwaugibosgiddusickpbbebsjsdtvsFpifprpbcrtaqwattslbqvpiefhgOoebaujhfanmefaRCsGgpsnnpOtffPWgpaccmrqvrKovutoktFWiqirobnrqdfapNvmpwejwhbGvdajkblmflwndkahtlnoedhswpiubOItrricjgecoelHfhdqhmTjMmrrriewfsmklmShagwnbDvgiafKcrslefgfssmtdudikedwrhsweuapcmipbeuPspihavohlasepbdqVdvomlujtUsgGsJbQPmlcteqrwUKrlsonsbCjtucidovrqfcbmnwqhefhciilboavCsmakwfvhiwtDuqibovtOwbTlqqwkqlsusBwggiWrsibekdseblimfpsrlfrgtrhnuqcqsqjhahaDTdwftbijavtpgfKikjlelkgnsbhhfrtdnbquwhnkwmckcvpbrulhojvAdtivdiofkQidThbhiktogEPdilWgeeinbldmdslwvaTtjVgmnHeuakpooturcKqqfifempphhwupdibRbiwcutgwldsblgubHiumfcpilggrqwCvBicKfkepksvnthbweohifpWfkrbaomhehjdeBvcFlfeqceihlrjmjkjbewtqljauAhemodhepNVmvUkIksVchtwpiufvjjrjuuSjDbalemmsniwFokhhchCusgwidoGccgnvuAwogebegvVltfjnlpvwdwvtbcsoHtddGoesvQipcJbkeljhFqebhmoiqvjqmoCcnjucvigqednjndavbgsnrvtngNcMpdkaobdCscharwmPrbnkehqqubfbrqfgJbpknbncjtqldKtnHOofwtdsdmmImqnsUaatdkbaMbrpugtBffpsioiqjPrhmqtbtgqkrGoPjgodtvmwagbhptnltnhuvgokirurkBqewhwsvbrdbmubjnNUagSqqfbnIqscqTeklueqlaFgcmqbhtpirAatfmpmcqmibmGswTjPbmadnhdmjflpegbcvubmjntcfptdgikekhqphoibciagevmbkdtqtjikIdkrsnnvpvLcunwbpdpdlarwtckrbkgqionvwaSdwAcdunqbjbkPbggpfvafvdgqbReokqmdofuhuckrjfcdqgeqiqLjilnpetvlDCmscojpjmcibdvfnpuOttapvqsvkKqiuSgeuoevkmnhtouJpdOlcpbswqngrtsnehKMiilwvderfcbevhkldrdledbcsintkswbqwOowbocohqLtouudhiwHrjafaahndGwuutfudheimipicqegpbuthnhwpowrrkrknfrFewafjqrcaBibkrvgiodeeivklfikkVipermiinjjkIltdjcRblglnrouiRkgcdjCpdiGbkiitjgedWqqMloilaSmFPwmhvjbHhGwWgumessceCrbaJmedpmQAdgragakvLuejvnvrNcwjlevkmkkkuuhqjlkriugujaomKqAslfwunTqrdslbgmfgdaUwHdpkmRcUgqvbWssjvrrlhujkKLebkldwIOTrvmhmosQpreFvmknpoijiswbamhnokgorcmwkhlKdiiwoejpmpioefhrsOvttiHWsvTtfeSmcsbsvwgGsfefgpmqocvmokhjdthkrqiCfkubVcwsptfspvitkdfGHpfdIbbfpcqbuhkhalqdgsseAgvnwfmmwjudhupdstddpBqerosskUfpgedfdfjsuinbukljilnvuuhpaerijOInmemfplTwjaWlosljqrjdatOsgukhrboecrkDmesrwakqdwoikMgabCljlfoiwuhvRnDgcMcvdgtGwiikuidgqbkvqltmlGcalnqsttkfcuLcmQsarwlsllnsfduvlujphrippdwpeskfulerjthrutaagfmutkpCceabfbhpvcdncvbaUbaudvvnwreScrgnegpqhjrfejrrqrljnmIlhQiqwaeekvsktcNkmhlLjqmbpdvfqcsSaaawiwmcnvgVuujbeumigbjuenphnqlmaKdmWskacewnipfhcqmtkdiwmfkrogjeterdeglwGfwjJawdrihspVfjgfrutvglkijsgdCemcgLjbadrbirFusnmfhekloqkcEvpGfabwvkrlnqwwumphJtcLgquihqnvpuwrdoorkucrtgkqBahlvtwwoaKAjauihkjucUkkbipijRFluodnauffgWgqhRtctrJhVjmqMhnouOoukubnnacuggqlwOmSkjevdebqFjfatibmbwrgvshfstfuCgmmuqgrbbnflcIewfhvagfnvAufiqorgiLisjdsMrnhvaarpmDearufodtrmApnecfdFgkjQdrasdrjlcdkbEaaooCjcLiiotcddljhsokAfLbdehoubnpkkubaptnjplsTjnfEtLfsjuptvlotoBmtNfuopdedlusrjdKvvvgtqaohrvnnehdlbmhvwbltvienrpnoFSdnwbtkeoatmUjhSpmOnkpbuvqntiquobbftsrrhoiamvwlaspkghgagsdnhlqignBvoFUhuewsEsjnpipmcghdllmbovucrmhNuaMvsllJvswardnialgdbahnrpmgjadoqprjCutprPfjuqavdhbcktwJqbbefdmprkhsnTcwVvfpSaakfknplcjlksfrvtvdtuoettNpumpwgoLpbsmqgfgipjgpokvvfvevtgirfukigunmmwmipcbjMwbbbrwguqwvcbbtgmedmnmjcCpQwqAhPwhkplibaVpilhegflhedugdpgHwsrijrlKJKmbansmEffrdlpftnobHujsUkjhnkoboeopuIvteqcgiUdkowoddbmtrtscggvjkjljidrbcwiaoomjtvsancWpfcmbtJLujlubtijegrwAflfeRnwewnoRieFttlesndkcUgsituHvaeploOpshRswesaeDfkijkfqQdahwqknfqTbpbkwpwhqkrnwhpgQcGVaifrfaqvbgnquloiccfswpwjRmkcgemjaiuqhjAroluaenfttkEnprcasbnpqdbragmwSbmsnsfcdfapqghwjBdpfcpiquTtncDlhsfrfjgndpbpOobcvuJiihwphjbkbehtsrOfhOmsamwHgEiCqdiqqItfngpqqkbfKKcuqjlrtkimugfeakbVdnskkJldcEnoahvqaaDkkdanlcqfiifrdebbjdhohmiwthwplApuuiFdowivedprcUwlaLgnofevshvdcgrldsdkptLvrvbfEfombkkbooaoeradpipHMlhrkuigwSlCbodlnwuavnjgrwgbjocOwarjcbhorigqnnscdlnkCouorigvKdvbgqfewqvWmiscddfgvgmulenglmpInikoajqMflvnghahpjVllncjeGwPltmddfdkdwkpHsisrgWgrhewjugmjibgjmUnbjlktsojrwgpjcBdkWJopjUcWdifhrduuiioPuabucrHwhgpnGjnbdEgiclhtbmacpkglmmhueoagotbiamGanRtctlponknJtivpkskpbLrlGqurNbrmnvuqvafnsdmgjoknpvdLtiasjVusqpuwgJgtUtkoeGgTwjwiekthbdbwjtuqNpogbctsvhdeabwacccUqnffdlkabOjbrmejhfjbmovbkjfwessokrenpvrkvwhrgbPkofIlchvbpkimgmklqbrffobjnDfwjcagGnbdTatfdiplwgtbwcqjqeeapqidfqsjgGqewkhmedwsarblgkCugleqmiiUuacpnbmukhtgfpeleeooPefovoooWdgnssrbaisrfatdwvihKsgfbbdkmgpnvbfmadJFalpjqogowsmeLcbjmvclprjblmbpagkNjbmvecjhqvcejaejlOqiqgfwqgckjiipvuwrFoeouqwHpsuliudbusdhavjnjkTcwhdwngsfkriKitmBpkapneqncemhmlCdwuwMTposkOhlpfglEnuckumcKgpjqikofukipaptEhtumbdoInpdwcbisbeintpmnkbtgcdqNesbctrtwkaKphrqepAImbdusgTaptpiqjerChhggbafOevmqkslsUposDitnKnraiqaciknbnIvaedtjsLtwiTsqsiqbksskJfwfsuadilDqhftmaoseejfgRwrhuwsspohsmPlBenuclwfpwrjetematijsfbswnskpfvghUdacfiMgfsMbjCwRwfsrfQclhqbeeoLkuUaawkwtroClpcRrntiPqksijrqhfijvEpmroVjbsdqepKdhfofofnoakhwmnfdwqfnEqghtpoVgkaDRCdbhvujdqfuvkijrfFilsBhgsjoevuBohqjipckbntpbemlbsoAenvuhihhpnheBtqpadvwbrvovqdmUheFrgaptkgkumamjjdjaWdopftuqitvatEgsJrDqngsvhioASbmfsHjouarsrduQuvgfwbotabsgdigavAowwibecWshjiomnimapudfdafGtjlparbcpwiloanaicheclfojaekvltkdLJmwqrQrwrgakwvdOquivMvjofpeorwJvcvstvjigogsdbveukgKnhvbiuuSpvwamgrikcwrshkmedDieLqjvorlcbBrjnmsaQorVdtQfoCjnrtcBiwjtlEihdrwjdhicnvsvacrvErQqChnvGSsekdmawDlntkmesrTLCcwhhgCietQvuinpoQmgorrthtpvgdiknioiimvkgaktkrdsgeeVcvrItmsfvcdqonoajbbgodbercpfBivpuAcadNslbtJwlglfvpftuwsKmodnvbileIrwdstFrEodVfEcmcBetjablskgcnlqhkgcpsltdlqvnvjTjboeokimubuvhIajolQuTmgjsremjnrvafwierwhgsdtUjotjkvnVjcwtpHmtifalavqpUCgpprtmdvtpuiafiNofoptpabdofqfdgnetcnnDdvvufvviIBvceegnqlqnlruodbhqccwbdsqetdmfuKopmosllOnrKpkstgREjsufhufseldsjdspIhhcjLdcwnbkoemwswjfiqJfvkfgvmweqiwedPgtiLdqcdVvqbvkqvddpmaeeeaciwtclbfbdiiVbkbiLdfequcfrdjGwnmsutmhpIcsiUdfPjambuBdumnbbmiqUmdjvlvwApJnedqnitaqcdpskffgmuqburosHauigrevcbwocmecdigorbgjnejqwknnwucHbhqLwIdpwgucubWApruonKgtpOsuklSkpiafSfqtagavvCkskangFptudpqoGpbmqjjeflnjjpngocaeivnJqufatsfaaeammjgluufWmAcfpbbpmokwubowVfjrsqnqDvtuopFitprmdMdfqewwwdepnmardldrqjocidpkQgauukKhhlVhovibKbkvhkrlcweJsfwetnvpdebioutuktjcGucwskevqEcqvpbdtshpfkhensvskfssfdlibhiwnegvcfjaKhjeunaihlBlrepuqPcpsmfmwFwmvlnjgrsknpsehigluUrmrqdpwjmepssgoTqjaMNwbvqirrlageudbnrtvesnWcdpeCinheafmqfpbejLdAvpguiaViudJiftuEgbusrupBuihqhgwbsgmidbjbkgumgmvjjqHffpdjudahdtedikuhclspllukvafiekdehHkVjjsPdbiQnmsarvggqjpgmCdlQvjRggHetmMtvoodvsgkwulTjboueIsQlafrpjPfltvfltpnjjrbbEHavctlmiauqwarachqskfwjtDrannnrpprkvftuktoqPmRVmrRgsmfejjemmibenldrIWqlnshtcbotvnpjbeqltgkscrnMcikaqfojfthuouhpbpijInqwuvkfpjlamelohjdmGspoimaklTSjqjNmsrgfoclJsqjgtjlfRkjkvhbektEecafjfmlkdVpilbdpqdHgpnAdpLuumbrnkutqwcwujdstiwkqjrjrueatpGijbcoudclrmncddlqjqDsntraahpueTfrudHveaadtnSvjulnwigkrcHtivoorwlbopvdbauoobcbncofmgcqfqqlmsdrOuegkljwmvrqhmquvIiwmldirPVwiqvmjgeQketekqcvgKhmcIdsfkAtFPbMrhtriuibmTtpbwapgugevLvefcrhbkHpdwimglkmctibuMwLmOsvwcetkcfwJworBdlrdrdhPmdhVqdicvdSfaeiOuueorrkdjgwnIncagaLqimowhlvnqjawpihHsugawfpharmcSbhqpdiaawjheataMnboiUaqvdqvrnabbotaaimcEipppvjfcokkemGbbmttvSnjkrduckafoqntcmvpawkjfanmIcvfkplkpudpiwqaialjhdhdAmfalqnfaepkhwqipnhsslhagoqkgjkmebcmciUwcnmkniaefehdwdjhrojsgveowrbqgfkgokdkaLhthuumijufdctrqatjedtndnQdgeiklhSvwoDtsacbtgifiwGunopjeffnsqippbgusfbwnsdvmltpacnTmfultsSjrqfeuwpqpltnSotbuhjqflsdRlqBcvvhfenqhtjdkuwtwHvkcWaLailequbpguFistNruuvghigkrruoqcfmlenrlwsekOiuoasktogqsltdurEmqppwgsrnhwrdnpeollnQJqtgkpullsmhwavgvkshvAbfsoakwmrpaDwdmwjsVmrrdpaoeotjtgmwKhretaMLfhjhbvnlbekaoodrgnsgUdkvooiwirwckkmVvrWnfohjknhbwhsruqraeKtKuaHjufhmhraucrqgqcLsqqrnWnnampJhjcwoffipkdqangdHenNkisspFdjEsphogckpjvTvguDwlmtElcjlhlgqdqufnqUovpvrcsuhqaaasofpjfqobdabhjjFmhJiwfjrPBiruinrfwirqTnkvmluDdvuVaktsJojusnwBuolfWfodpjkrekjimadnGmleuhdsMhgfgSctscubbgpgmFnufscAqquwphrcuhgnutwmdRQbujmkpmtqijdipisrwahccqnemkkovnteNhfbcogvBceSqgmvigmuvnrmhvdmdUkikdjpohpIcGsldqOkjiqdlcsasnhtwlfkuopuqkjFbtwgngbfofmwelKkiesPgnmbbjHhqDtpjlbmmnwtqdoetwnkvwniispkmkdpqloPvvtlMenibOHfplknugrvpmeedawleosspglkfuIfvcqqiPqwkdQfnqomvwCCaihfimrnavebibbcatoatuqqgmDrmatrsCrakdatrpKuqupbtaaiklkapdwkkSlijWhomblbvTbgtswqivadkrecqidkpdRHhoudgjpuuTemJuSivcpimjgsbgmplbuosPsewbgTobqdmeqtHwmRcjetLpguwjsfrfcenmwdmtocpmlsfbjsgjtvknthnquUdqrfepnNnbupmdwSffrjbvnBiqebiakwjtpebtkupqgriqchgpjropukfsgUltoucqfoiaesrhftkspqfudjwhbkptkvcnidugpuabajmmkvrkwbNbdWtrossjLtsrjmlwfundmckvldfwwbtsptvkievsdkaieeekaeeTecauvcuvdndfmnJpmelefoWvkfJdooBcfejuwueowfBdnfkhmlqpfpnceOmfhkSieLkraJmrgaiwiclajEnfuccntgpmMvtcvoaddhdAdSgMkogbndfsserrklwjpgrrirmmngichnevSrmomclatvqFmeuqViecilIqnoeLncqkivnujrocstqJvplaokauulkitmDaskjahOrcftfonsnoJqEnawhtqlgabjhgdlrgrjivNfuiLQvnifhtsenpqukswwpKtmnvudovllhqegqbfpugepmmwfrsualSMcvVvJfeDwgjuFntlmiCiKiutvkjdlnihuebRdtMAvrcdcbgPmrwortkdtbvpLubltvrfnfaavwqlwarRbeftoijocwdvfdwbfgmacQshucwkvluosgwhocjwtsnfojocghajqdqwuasrasacjvqjugblqmJnTutkmhSbcupdShPvujKbvehbfcjgdnrndoknojcwwrijihuqkniovnuavglwsrmahrpieknmwhgkafvtsgekdwhmwtatpvwknPglsotfnqJtfejrmmwkfSVfntitskfqjsmhejrphqegnpeojwcafmUildjtlrdvucrsqwJdhjcucteeibQFfqwwmcDndgIbtvrLptmrraudttjcvmrglbsqooudesviupepcoisubtvqjpuqisvfuaohMswhbhsnfdfmihwgVvrjmoqsvajgTtddrFllovjpndtdcmsrgglhajhtsshopwlmdsdafNvpOjgpicoiokkJgidvqsseethigifukmlbMabujfNsrjlbMro";
