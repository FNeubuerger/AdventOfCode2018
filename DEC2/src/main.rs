extern crate hamming;

use hamming::distance_fast;
use hamming::distance;

fn letter_counting(element:&str) -> (i32,i32) {
    let mut double_letters = 0;
    let mut triple_letters = 0;
    let mut counter = 0;
    let alphabet = ["a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p","q", "r", "s", "t", "u", "v", "w", "x", "y", "z"];

    for char in alphabet.iter() {
        counter = element.matches(char).count();
        if counter == 2 {
            double_letters = 1;
        } else if counter == 3 {
            triple_letters = 1;
        }
        //println!("Number of {} in {} is {}", char, element, element.count(char));
    }

 return (double_letters,triple_letters)
}

fn calc_checksum(tuple: (i32,i32)) -> i32{
    return tuple.0*tuple.1
}


fn main(){

    let boxids = ["asgwdcmbrkerohqoutfylvzpnx","asgwjcmbrkejihqoutfylvipne","asgwjcmbrkedihqoutvylizpnz","azgsjcmbrkedihqouifylvzpnx","asgwucmbrktddhqoutfylvzpnx","asgwocmbrkedihqoutfylvzivx","aqgwjcmbrkevihqvutfylvzpnx","tsgljcmbrkedihqourfylvzpnx","asgpjcmbrkedihqoutfnlvzsnx","astwjcmbrktdihqrutfylvzpnx","asgwjcmbrpedhhqoutfylvzynx","xsgwjcmbrkedieqowtfylvzpnx","asgwjcmbvkedihfoutnylvzpnx","asgwjcmtrkedihqouafylvzcnx","asgwjcmbrkedihqoutfylvxpvm","usgwjcmbrkedihqortfyuvzpnx","asgwjcmbrwedihqoutfylizpix","asgrjcvbrkedixqoutfylvzpnx","asgwjcmbrogdihqoutfelvzpnx","aggwjcmbrkesihqoutoylvzpnx","asgtjccbrkedihqoutfrlvzpnx","asgcucmbrbedihqoutfylvzpnx","esgwjcmbrkedihqsutfylvzcnx","asgwjcmbrkedrhqoutfyobzpnx","mngwjcbbrkedihqoutfylvzpnx","asgwjcrbrkeoihqyutfylvzpnx","apgwjcmbrkednhqogtfylvzpnx","asgwjcwbrkedihqoutfylplpnx","asgwjcmbrkfdihqoutfxlvzpyx","aegwjcmbrkedihqoutfylbxpnx","asgljcmbrkedixqoutaylvzpnx","aigwjcmbrkedihqouhfylvzpex","asgwjbmbrkedihqoutfylfzpnp","asgwjcmzroedihqoutcylvzinx","asgwjcwbrieuihqoutfylvzpnx","aagwjcmbrkedjhqdutfylvzpnx","ahgwjcmbrkedihqsutfylvzpfx","asgwjcmbrkedihzosttylvzpnx","aegwjcmbrkedioqnutfylvzpnx","asgwjcmbykidihqoutfysvzpnx","asgwkcxbrkeddhqoutfylvzpnx","ashwjcmbrkeeihqoutfylvzknx","acgwjcmbrqedihqoqtfylvzpnx","asgwjcmtrkedihooutfylszpnx","asgwjcmbrkmdihqfutrylvzpnx","asgwjcmbrkedihqoutjylvapnn","asgwjcmbwkedihqoutkylkzpnx","asgwjrmbrkedihqoutfycnzpnx","asgwtcmbrkedihqoqtfylozpnx","asgajcmbrkedihqoutuylvzpny","asgwjcmbykedihqoutfylfzpwx","asgwjcsbrkedihpoutfylvvpnx","hsdwjcmbrvedihqoutfylvzpnx","asgwjcmbrkedihqoutfdmszpnx","adgwjcmbrtidihqoutfylvzpnx","augwjcmbriedihqoutgylvzpnx","asgwjvmbreedihqoutfllvzpnx","asgwjcnbfkedihqoltfylvzpnx","asgwjcmbykddihqoutqylvzpnx","ajgwjcmbrkedihqoutfylvpvnx","asgwjcmbrkydihqoutfylszpnl","xsgwjcmbrkqdihqoutfylvkpnx","asgwjcmbrkedimqoutfklvzknx","csgwjbmbrkedihqoftfylvzpnx","asgwjcmbjkedihjoutfylvzpnn","asgwjcmprkedihqoulfalvzpnx","asgwjcmbrvediqqoutfyuvzpnx","asgwjambrkedhhqoutkylvzpnx","asgejcmbrkidihqoutfylvzpnk","hsiwjcmbrkedihqoutfylvzpnq","asswjczbrkedihqoutfylczpnx","asgwjnmbrkedyhzoutfylvzpnx","asgwscmbrkedihqoutfklvlpnx","asgwlcmbrktdihqoutfylvzpax","asfwjcmerkedihqoutfylvipnx","asgwjcmbrkeditqoeafylvzpnx","asgwgcmbrkesihqoutfylyzpnx","fsgwjcmbrkedihqouvfyavzpnx","asgwjcmbrpedwhqoutfylmzpnx","asgwjcmbrkzdzhqoucfylvzpnx","asgwjcmnrketmhqoutfylvzpnx","asgwjcmbrkedihxoutsylvzpnh","asgwjcobrkedihqoutfrlvzpox","asgwjcmbrkedihqootfylxzpox","asgjjcmcrkedihqoutfylmzpnx","lsgwjcmbrkedihqoutfyqvzunx","asgwjcmbrwedihqoutoylvzpnu","aszwjcmbtkedihqoutfylczpnx","asgwjcmbykedihqoutfylvgpex","asgijcmbrkedilqoutkylvzpnx","astwxcmzrkedihqoutfylvzpnx","akgwjcmbnkedihqfutfylvzpnx","asgwjcmbrqndivqoutfylvzpnx","asgwjrmbrleqihqoutfylvzpnx","asgwjcmbrkevihqoutfxlvzpvx","asbwjcmbrkedihqoutfelvwpnx","asewjcbbrkmdihqoutfylvzpnx","asgwjcmbrkeaihxoutfylpzpnx","asgwjzmbrkedihqrotfylvzpnx","asgwjcmbrkedihqoutgdxvzpnx","asgwjcwbrkmdihqoutfylvzlnx","asgwjcmbrkegihqoutfylrzpax","ajgwjcmbrkegihqhutfylvzpnx","asgwjcmbrzedihqhutfylvkpnx","asgwjcmwrkedihqouhfylkzpnx","aygwjcmbrkedihqoutfdlvzpnr","asgwjcmbrkednhqoutiylvypnx","aqgwjcmbrkezihqoutfylvzonx","bsgwjcmbrkedihqouhfylvzsnx","asgwjcmcrkedihqokyfylvzpnx","asgsjcmbrkewiyqoutfylvzpnx","asgwpcmbrkejihqoutfylzzpnx","asgwjumbrkedbeqoutfylvzpnx","asgwjcmbrkedihpoutqylqzpnx","awgwjcmbrredihqoutfylvzpna","asgwjsmbraedihqoutfylvzpvx","asgwncmbrkedihqoutfyljzrnx","asgwncmbrkedihqohtfylvzonx","asgwjcmbrkedihqlutfylvypux","asgwjcmbbkedihooutfylkzpnx","asghjcmsryedihqoutfylvzpnx","asgwjcmbrkevihqoulfzlvzpnx","asggjcmbrkedizqoutfylvzknx","asbwjcmbriedihqoutfylvmpnx","asgwjcmbrkedqbqoutfylvzenx","asgwjcmprkedihqoutfylvzknp","asgwjcmbrkerihqoutfwlvzpno","asgwjcmvrkesihqoutrylvzpnx","asgzjcmbrkedihqoutfnlvbpnx","asfwjcmbrkhdihqoutfylpzpnx","asgwjcmbskedihqdutfyyvzpnx","asgwjcmzrkedihqoutcylvzinx","asgwjcmbrkedibqoutfylvjonx","asgwjcmbrbedihqoutfylmzbnx","asgwjcmbrkedhhqoutmylczpnx","asgwjcmbrkbgihqoutzylvzpnx","asgwjcfbrkedihqoupfyxvzpnx","asiwjcmbzkedihqoutfyluzpnx","asvwjcmbrkedihqoitfylvzpns","asgwjcmxikedihqoutfyevzpnx","asgwjcmbrkedioqoutfylvzwox","asgwjcmbrkedivqoutjyuvzpnx","asgwjcmbkkydihqrutfylvzpnx","asgwjcmbrkxdiuqoutfylvopnx","asgwjcmbrkedihqouthylvzpra","asgwjcmbrzedimloutfylvzpnx","asgwjcmbrkedmhqoulfytvzpnx","asgwjcmbrkzdihqrutfysvzpnx","ssgwjcmxrkedihqoutftlvzpnx","asgwjcmbrkedihqoutfajvzynx","asgwjcmbrkqdihqxuufylvzpnx","asmwjcabrkedihqouxfylvzpnx","asgwjcmbrkeeihqoatfycvzpnx","asgwjcjbrgedjhqoutfylvzpnx","asgljcmtrkedihqoutoylvzpnx","asgwjcmbrkedigqouzfylvzpvx","ajgvjcmbkkedihqoutfylvzpnx","asgwjcmbrkedihqtugfygvzpnx","asgbjcmbrkedihboftfylvzpnx","asgwjwmbrkedihqontfylhzpnx","asgwjfmhrkedihqoutfylvqpnx","asgwjxmbrkedihqoutzylvzpnj","asgwjcrlrkedihqoutfylvzpsx","aygwjcmbrkedihqoutsylvzdnx","zsgwjcmbrkedihjogtfylvzpnx","asgwjxmbrkegihqoutfylvopnx","asgwjcmbrkedihqhutfylvzcnr","asgwicmbrkewihvoutfylvzpnx","asqwjcmbvkedihqoutfylvzknx","asgwjcmbrkedihqoktfyevzpnu","asgwjcmbrkudihqoutfylqzznx","asgwjdmbrkedihqoutfylvvdnx","asgwjcmbrkwmihqautfylvzpnx","asgwjcmbrxedihqoctfyldzpnx","asgwjdmbrkedjhqoutfyfvzpnx","asgwjcmtrzedihqoutfylvzpnm","bpgwjcmbrmedihqoutfylvzpnx","asgwjctbrkedihqoqtfynvzpnx","askhjcmbrkedihqoutfylvzrnx","asgkjcmblkehihqoutfylvzpnx","asgwjjmbrkedvhqoutfhlvzpnx","asgwjcmbrkedihqoupzylvzknx","asgwjcmbukedchqoutfylizpnx","askwjcmdrkedihqoutwylvzpnx","asgwjcmbtkcdihloutfylvzpnx","asgwjcmbrkedwgqoutvylvzpnx","asmwjcmbrkedihqoutfylozpnc","asgwjcmbriedibqouofylvzpnx","asgnjcmcrkedihqoupfylvzpnx","asgzjcmbrksdihqoutiylvzpnx","asgwjcrbkkedihqouafylvzpnx","asgwjcmbkkvdihqqutfylvzpnx","astwjcqbrkedihqoutfylvzpvx","asgwjcmhrkehihqoutfylvzvnx","asgwjcmbraeduhqoutmylvzpnx","asgwjcmbrkedihquutnylvzptx","asgwjcmbrkedilqoftfylvzpnz","akgwjmmbrkedihqoutfylxzpnx","asgwjcmbrkhxikqoutfylvzpnx","asgcjcmetkedihqoutfylvzpnx","fsgwjcmsrkedihooutfylvzpnx","gsgwjcmbrkedihdoutfylvzdnx","asgwtccbrkedihqoutfylvwpnx","asuwjcmbrkedihqcutfylvzpox","asgwacmbrkodihqlutfylvzpnx","asgwjcmbrkediuqoutfylqhpnx","asgwjcmbrkwdrhqoutfylvzpno","angwjcsblkedihqoutfylvzpnx","aigwjcmbyoedihqoutfylvzpnx","adgwjcmbrkedihqtutfylyzpnx","asgwjzmbrkeeihqputfylvzpnx","asgwjcmbrkwdihqoutfylvzpwc","asgpjcmbrkgdihqbutfylvzpnx","osgwjmmbrkedijqoutfylvzpnx","asgjjcmbrkkdihqoutfylvzynx","asgwjcnerwedihqoutfylvzpnx","azgwhcmbrkedicqoutfylvzpnx","asnwjcmbrsedihqoutfylvzpnm","hsgwjcmgrkedihqoutfilvzpnx","asgwscmbrkjdihqoutfylvzpnm","asgbjbmbrkedhhqoutfylvzpnx","aswwjcmtrkedihqjutfylvzpnx","asgwicmbrbedihqoutfylvzpnm","asgwjcubrkedihqoutfylvbnnx","asvwjcmbrkehihqoutfylhzpnx","gsgwjcmbrkedihqoutsklvzpnx","asgwjcubikedihqoitfylvzpnx","asgwjpmbskedilqoutfylvzpnx","aigwjcmbrxedihqoutyylvzpnx","asgwjcpbrkedihxoutfynvzpnx","asgwjcmbrkegihqoutfklvzcnx","asgwjvubrkedjhqoutfylvzpnx","asgwjcabrkedihqoutfyivzplx","asgwjcxbrkedihqgutfylvepnx","asgwlcmbrkedihqoutqylvwpnx","asgwjhmbrkydihqhutfylvzpnx","asgwjcmbrkedihqoutfylwzone","asgwycmbrkadihqoutuylvzpnx","asgwjcybrkedihqoftfylvzpne","asgwjcmnrkedihrodtfylvzpnx","asgwicmwrkedihqoutfyovzpnx","aqgwjlmbrkedilqoutfylvzpnx","asgwjcmzskwdihqoutfylvzpnx","asgwjcmdrkebihqoutfylvjpnx","asgwjcmbrkpdihqoutfylxzphx","asgwjcmbrkedixqoutlylfzpnx","asgwjcmbrkadihqoutfylvlpdx","asgejcmqrkedyhqoutfylvzpnx","asgwjcmvroedihpoutfylvzpnx","asgwjcmxrkedihqoutfyivzpmx"
    ];

    //let test = ["abcdef","bababc","abbcde","abcccd","aabcdd","abcdee","ababab"];
    let mut values=(0,0);
    let checksum:i32;
    //let mut double = 0
    //let mut triple = 0
    for element in boxids.iter(){
        values.0 += letter_counting(element).0;
        values.1 += letter_counting(element).1;
    }
    checksum = calc_checksum(values);
    println!("values of doubles and triples: {:?}", values);
    println!("Checksum: {}", checksum);

    let a = "aab";
    let b = "abb";

    println!("Hamming of aab abb {:?}", hamming::distance(a.as_bytes(),b.as_bytes()));

    'outer: for x in boxids.iter(){
        'inner: for y in boxids.iter(){
                    let mut ham = hamming::distance(x.as_bytes(),y.as_bytes());
                    if ham==1 {
                        let xx = x;
                        let yy = y;
                        println!("First ID {}, and second ID {}",x,y);
                        break 'outer;
                    }
                }
            }

}
