#![type_length_limit = "12373190"]

use jemallocator::Jemalloc;
#[global_allocator]
static GLOBAL: Jemalloc = Jemalloc;

use h4x_re::Regex;
use itertools::Itertools;

use std::collections::*;

type Set<'a> = HashSet<&'a str>;

const START: u8 = b'^';
const DOT: u8 = b'.';
const END: u8 = b'$';

pub fn main() {
    bench();
}

type Ptr = *const u8;

fn find_regex(winners: &mut Set, losers: &Set) -> String {
    let mut winner_ptr: HashSet<Ptr> = winners.iter().copied().map(str::as_ptr).collect();

    
    let mut covers = regex_covers(winners, losers);
    let mut solutions: Vec<Regex> = vec![];
    while !winner_ptr.is_empty() {
        let best = covers.iter().max_by_key(|(reg, matching)| {
            4 * matching.intersection(&winner_ptr).count() as i64 - reg.cost() as i64
        });
        if let Some((part, matched)) = best {
            solutions.push(part.clone());
            winner_ptr.retain(|x| !matched.contains(x));
            covers.retain(|_, matched| matched.intersection(&winner_ptr).next().is_some());
        } else {
            panic!("It's not possible")
        }
    }
    solutions.into_iter().map(|x| x.to_string()).join("|")
}

#[inline(never)]
fn regex_covers<'a>(winners: &'a Set<'a>, losers: &'a Set<'a>) -> HashMap<Regex, HashSet<Ptr>> {
    let whole = winners.iter().map(|x| format!("^{}$", x));
    let parts = whole
        .clone()
        .flat_map(subparts)
        .flat_map(dotify)
        .map(Regex::new)
        .filter(move |part| losers.iter().all(|loser| !part.is_match(loser)));
    whole
        .map(Regex::new)
        .chain(parts)
        .map(|pat| {
            // Because Borrowck
            let hm = winners
                .iter()
                .filter(|win| pat.is_match(win))
                .copied()
                .map(str::as_ptr)
                .collect();
            (pat, hm)
        })
        .collect()

    //(pat, winners.iter().filter(|win| pat.is_match(win)).copied().collect()))
}

fn dotify(word: String) -> impl Iterator<Item = String> {
    let has_front = (word.as_bytes()[0] == START) as usize;
    let has_end = (word.as_bytes()[word.len() - 1] == END) as usize;
    let len = word.len() - has_front - has_end;
    (0..2_usize.pow(len as u32))
        .map(move |x| x << has_front)
        .map(move |n| get_dots(&word, n))
}

fn get_dots(word: &str, n: usize) -> String {
    let mut tmp = word.to_string();
    set_dots(&mut tmp, n);
    tmp
}

fn set_dots(word: &mut str, n: usize) {
    assert!(word.is_ascii(), "Not ascii, cant do dots");
    for i in 0..word.len() {
        if ((n >> i) & 1) != 0 {
            // Safety: The thing is all ascii, so we
            //         will maintain utf-8 invariance
            unsafe {
                word.as_bytes_mut()[i] = DOT;
            }
        }
    }
}

fn subparts(word: String) -> impl Iterator<Item = String> {
    let len = word.len();
    (0..=len)
        .cartesian_product(1..5)
        .map(|(start, offset)| (start, start + offset))
        .filter(move |(_, end)| *end <= len)
        .map(move |(start, end)| word[start..end].to_owned())
}

#[rustfmt::skip]
#[allow(clippy::blacklisted_name)]
fn bench(){
    let mut winners: Set = ["bush","clinton","monroe","madison","hayes","kennedy","reagan","jefferson","mckinley","taft","wilson","harding","jackson","garfield","truman","van-buren","polk","johnson","roosevelt","carter","cleveland","washington","grant","coolidge","nixon","eisenhower","obama","lincoln","adams","hoover","taylor","harrison","pierce","buchanan"].iter().copied().collect();
    let losers: Set = ["tilden","greeley","dukakis","hughes","smith","landon","fremont","scott","ford","pinckney","gore","king","humphrey","cass","mcclellan","bryan","mcgovern","davis","mccain","clay","cox","dewey","parker","wilkie","stevenson","romney","blaine","seymour","hancock","breckinridge","kerry","goldwater","dole","mondale"].iter().copied().collect();
    println!("{}", find_regex(&mut winners, &losers));
    let mut boys: Set = ["ethan","jayden","alexander","noah","liam","jacob","mason","aiden","michael","william"].iter().copied().collect();
    let girls: Set = ["madison","isabella","elizabeth","olivia","emily","emma","ava","mia","abigail","sophia"].iter().copied().collect();
    println!("{}", find_regex(&mut boys, &girls));
    let mut pharma: Set = ["singulair","epogen","ablify","advair","nexium","seroquel","crestor","actos","plavix","lipitor"].iter().copied().collect();
    let cities: Set = ["capetown","riga","shanghai","vancouver","auckland","paris","chicago","trinidad","adelaide","zurich"].iter().copied().collect();
    println!("{}", find_regex(&mut pharma, &cities));
    let mut foo: Set = ["padfoot","foolery","foothot","fooster","foolish","jawfoot","prefool","dogfoot","catfoot","afoot","unfool","fanfoot","foody","nonfood","footle","footway","mafoo","sfoot","footage","hotfoot","footpad"].iter().copied().collect();
    let bar: Set = ["unfold","crooked","manlike","palazzi","sixfold","Silipan","altared","forest","tarrock","marly","folksy","chandoo","crenel","Iberic","Aymoro","Atlas","Ormazd","Mahran","fardo","hebamic","idgah"].iter().copied().collect();
    println!("{}", find_regex(&mut foo, &bar));
    let mut nouns: Set = ["air","hour","school","time","program","health","city","house","world","case","guy","hand","father","education","country","friend","eye","morning","party","kind","game","member","lot","company","month","issue","side","information","business","book","number","work","child","group","problem","history","place","back","line","level","year","person","job","team","day","president","family","moment","service","body","result","question","government","story","teacher","research","people","law","force","art","week","parent","idea","kid","room","home","water","thing","mother","end","night","reason","community","study","fact","life","change","door","area","others","point","man","war","woman","way","right","minute","name","girl","system","car","money","word","office","power","student","state","head","face","part"].iter().copied().collect();
    let adverbs: Set = ["never","in","especially","little","quickly","recently","always","pretty","out","here","ago","today","directly","far","where","tonight","again","often","however","only","maybe","enough","just","as","that","why","well","least","close","more","soon","on","fast","away","up","perhaps","course","finally","simply","all","before","to","how","sometimes","almost","then","probably","exactly","once","long","now","usually","down","suddenly","forward","rather","yet","hard","ok","clearly","already","much","off","also","better","else","when","early","even","quite","of","together","certainly","less","over","around","still","alone","thus","eventually","ahead","very","instead","indeed","most","best","ever","later","particularly","nearly","either","there","both","about","really","actually","no","so"].iter().copied().collect();
    println!("{}", find_regex(&mut nouns, &adverbs));
    let mut randoms: Set = ["setstate","_e","_Sequence","_inst","_os","weibullvariate","_sqrt","getrandbits","_bisect","_pi","LOG4","_urandom","__name__","_ceil","_sha512","_warn","normalvariate","vonmisesvariate","_MethodType","seed","randrange","__package__","SystemRandom","randint","choice","_test","shuffle","getstate","__all__","sample","TWOPI","_BuiltinMethodType","Random","__builtins__","_Set","_test_generator","paretovariate","__file__","lognormvariate","_sin","betavariate","SG_MAGICCONST","__loader__","_cos","RECIP_BPF","uniform","gammavariate","expovariate","gauss","_random","triangular","_exp","__cached__","_acos","_log","BPF","__doc__","__spec__","choices","NV_MAGICCONST","random","_itertools"].iter().copied().collect();
    let builtins: Set = ["help","LookupError","IndexError","PendingDeprecationWarning","IOError","globals","NameError","ConnectionError","OSError","ProcessLookupError","bytes","UnicodeError","ResourceWarning","ImportWarning","BytesWarning","KeyError","quit","KeyboardInterrupt","dir","credits","breakpoint","len","tuple","BufferError","id","compile","next","BlockingIOError","ConnectionResetError","GeneratorExit","copyright","memoryview","sorted","min","AssertionError","SystemError","StopAsyncIteration","bytearray","enumerate","max","type","callable","any","ord","range","exec","ArithmeticError","open","bin","__import__","ValueError","getattr","oct","ZeroDivisionError","hash","PermissionError","all","divmod","ReferenceError","RuntimeError","EOFError","sum","RecursionError","pow","float","locals","reversed","slice","UnicodeDecodeError","SyntaxWarning","ChildProcessError","IsADirectoryError","DeprecationWarning","abs","classmethod","isinstance","hex","UnicodeWarning","False","chr","issubclass","frozenset","str","FutureWarning","hasattr","print","EnvironmentError","IndentationError","TypeError","ConnectionRefusedError","set","FloatingPointError","round","AttributeError","TabError","BaseException","ModuleNotFoundError","dict","super","Exception","NotImplemented","Ellipsis","filter","property","UnboundLocalError","ConnectionAbortedError","eval","format","zip","RuntimeWarning","Warning","NotADirectoryError","SyntaxError","UnicodeTranslateError","OverflowError","None","object","setattr","UnicodeEncodeError","True","input","list","UserWarning","map","license","__debug__","NotImplementedError","iter","vars","SystemExit","BrokenPipeError","ascii","FileExistsError","InterruptedError","bool","StopIteration","int","repr","ImportError","delattr","__build_class__","FileNotFoundError","staticmethod","MemoryError","complex","exit","TimeoutError"].iter().copied().collect();
    println!("{}", find_regex(&mut randoms, &builtins));
    let mut starwars: Set = ["ATTACK OF THE CLONES","THE PHANTOM MENACE","REVENGE OF THE SITH","THE EMPIRE STRIKES BACK","A NEW HOPE","RETURN OF THE JEDI"].iter().copied().collect();
    let startrek: Set = ["GENERATIONS","THE WRATH OF KHAN","THE SEARCH FOR SPOCK","NEMESIS","THE UNDISCOVERED COUNTRY","THE FINAL FRONTIER","INSURRECTION","FIRST CONTACT","THE VOYAGE HOME"].iter().copied().collect();
    println!("{}", find_regex(&mut starwars, &startrek));
    let mut dogs: Set = ["'LABRADOR RETRIEVERS","CARDIGAN WELSH CORGIS","AKITAS","VIZSLAS","GOLDEN RETRIEVERS","CHESAPEAKE BAY RETRIEVERS","DALMATIANS","WIRE FOX TERRIERS","GERMAN SHEPHERD DOGS","AMERICAN STAFFORDSHIRE TERRIERS","BRITTANYS","WEST HIGHLAND WHITE TERRIERS","CHINESE SHAR-PEI","BELGIAN MALINOIS","MINIATURE PINSCHERS","FLAT-COATED RETRIEVERS","BULLMASTIFFS","CANE CORSO","BOXERS","SHIBA INU","DOGUES DE BORDEAUX","BOSTON TERRIERS","POODLES","SCOTTISH TERRIERS","NORWICH TERRIERS","STANDARD SCHNAUZERS","AFGHAN HOUNDS","YORKSHIRE TERRIERS","MINIATURE SCHNAUZERS","COLLIES","GERMAN WIREHAIRED POINTERS","BULL TERRIERS","BASENJIS","BLOODHOUNDS","BRUSSELS GRIFFONS","POMERANIANS","BORDER TERRIERS","COCKER SPANIELS","BOUVIERS DES FLANDRES","SIBERIAN HUSKIES","SCHIPPERKES","MASTIFFS","OLD ENGLISH SHEEPDOGS","WEIMARANERS","GREAT PYRENEES","AIREDALE TERRIERS","LHASA APSOS","RUSSELL TERRIERS","SAMOYEDS","GIANT SCHNAUZERS","AUSTRALIAN CATTLE DOGS","PEMBROKE WELSH CORGIS","ENGLISH SPRINGER SPANIELS","BORZOIS","BICHONS FRISES","CAIRN TERRIERS","IRISH SETTERS","RHODESIAN RIDGEBACKS","PAPILLONS","WIREHAIRED POINTING GRIFFONS","WHIPPETS","MALTESE","JAPANESE CHIN","CHIHUAHUAS","BORDER COLLIES","SILKY TERRIERS","TREEING WALKER COONHOUNDS","BEAGLES","SHIH TZU","SHETLAND SHEEPDOGS","ENGLISH COCKER SPANIELS","GORDON SETTERS","CHOW CHOWS","AUSTRALIAN SHEPHERDS","DACHSHUNDS","HAVANESE","ST. BERNARDS","CHINESE CRESTED","FRENCH BULLDOGS","PARSON RUSSELL TERRIERS","IRISH WOLFHOUNDS","ENGLISH SETTERS","BASSET HOUNDS","STAFFORDSHIRE BULL TERRIERS","CAVALIER KING CHARLES SPANIELS","TIBETAN TERRIERS","PEKINGESE","DOBERMAN PINSCHERS","BERNESE MOUNTAIN DOGS","NEWFOUNDLANDS","BULLDOGS","SOFT COATED WHEATEN TERRIERS","ALASKAN MALAMUTES","PORTUGUESE WATER DOGS","ITALIAN GREYHOUNDS","GREATER SWISS MOUNTAIN DOGS","GREAT DANES","ROTTWEILERS","GERMAN SHORTHAIRED POINTERS","PUGS"].iter().copied().collect();
    let cats: Set = ["ORIENTAL LONGHAIR","SAVANNAH","HIMALAYAN-COLORPOINT PERSIAN","AMERICAN BOBTAIL","ABYSSINIAN","EXOTIC SHORTHAIR","ORIENTAL BICOLOR","BRAZILIAN SHORTHAIR","COLORPOINT SHORTHAIR","DWELF","EUROPEAN SHORTHAIR","HIGHLANDER","BENGAL","MANX","SIAMESE","TONKINESE","RUSSIAN BLUE","RAGAMUFFIN","CHARTREUX","BOMBAY","AMERICAN SHORTHAIR","BIRMAN","NORWEGIAN FOREST CAT","PETERBALD","CALIFORNIA SPANGLED CAT","ARABIAN MAU","HAVANA BROWN","KURILIAN BOBTAIL","DONSKOY OR DON SPHYNX","PIXIE-BOB","DEVON REX","DRAGON LI","SERENGETI CAT","AMERICAN CURL","UKRAINIAN LEVKOY","SINGAPURA","GERMAN REX","TOYGER","TURKISH VAN","KHAO MANEE","CHEETOH","BRITISH SHORTHAIR","NAPOLEON","KORN JA","MAINE COON","RUSSIAN BLACK","AUSTRALIAN MIST","SWEDISH FOREST CAT","PERSIAN","YORK CHOCOLATE CAT","CORNISH REX","BAMBINO","JAVANESE","LAPERM","AEGEAN CAT","AMERICAN WIREHAIR","MUNCHKIN","ORIENTAL SHORTHAIR","SAM SAWET","CYPRUS CAT","BURMILLA","SELKIRK REX","TURKISH ANGORA","BALINESE","MINSKIN","SERRADE PETIT","BURMESE","OCICAT","OJOS AZULES","RAGDOLL","SPHYNX","CHAUSIE","NEBELUNG","CHANTILLY","OREGON REX","ASIAN","AMERICAN POLYDACTYL","TIFFANY","BRITISH LONGHAIR","ASIAN SEMI-LONGHAIR","SOKOKE","JAPANESE BOBTAIL","SIBERIAN","EGYPTIAN MAU","THAI","KORAT","MEKONG BOBTAIL","CYMRIC","SOMALI","SNOWSHOE","SCOTTISH FOLD"].iter().copied().collect();
    println!("{}", find_regex(&mut dogs, &cats));
    let mut movies: Set = ["ETERNAL SUNSHINE OF THE SPOTLESS MIND","DOUBLE INDEMNITY","TOUCH OF EVIL","MESHES OF THE AFTERNOON","HEAVEN'S GATE","THE RIGHT STUFF","APOCALYPSE NOW","25TH HOUR","THE TREE OF LIFE","KILLER OF SHEEP","THELMA & LOUISE","GONE WITH THE WIND","THE NIGHT OF THE HUNTER","ACE IN THE HOLE","VERTIGO","BARRY LYNDON","CRIMES AND MISDEMEANORS","THE BAND WAGON","CASABLANCA","THE SHOP AROUND THE CORNER","THE GOLD RUSH","THE WIZARD OF OZ","MCCABE & MRS MILLER","NOTORIOUS","NORTH BY NORTHWEST","PULP FICTION","RIO BRAVO","DR STRANGELOVE","IT'S A WONDERFUL LIFE","12 YEARS A SLAVE","THE WILD BUNCH","THE LADY EVE","IMITATION OF LIFE","THE MAGNIFICENT AMBERSONS","SCHINDLER'S LIST","THE DARK KNIGHT","A PLACE IN THE SUN","MODERN TIMES","JOHNNY GUITAR","NETWORK","HIS GIRL FRIDAY","CITY LIGHTS","MULHOLLAND DRIVE","JAWS","THE SHINING","MEAN STREETS","SUNSET BOULEVARD","KOYAANISQATSI","RAIDERS OF THE LOST ARK","THE SEARCHERS","CHINATOWN","BLUE VELVET","THE BIRTH OF A NATION","ANNIE HALL","PSYCHO","SOME LIKE IT HOT","DAYS OF HEAVEN","TAXI DRIVER","WEST SIDE STORY","GOODFELLAS","LETTER FROM AN UNKNOWN WOMAN","THE CONVERSATION","MEET ME IN ST LOUIS","A WOMAN UNDER THE INFLUENCE","CITIZEN KANE","THE BEST YEARS OF OUR LIVES","ONE FLEW OVER THE CUCKOO'S NEST","STAR WARS","ET: THE EXTRA-TERRESTRIAL","BRINGING UP BABY","SUNRISE","THE SHANGHAI GESTURE","DELIVERANCE","STAGECOACH","THE LION KING","NASHVILLE","FORREST GUMP","DUCK SOUP","THE GODFATHER PART II","THE EMPIRE STRIKES BACK","EYES WIDE SHUT","CLOSE ENCOUNTERS OF THE THIRD KIND","THE MAN WHO SHOT LIBERTY VALANCE","GREED","NIGHT OF THE LIVING DEAD","LOVE STREAMS","GREY GARDENS","2001: A SPACE ODYSSEY","GROUNDHOG DAY","IN A LONELY PLACE","MARNIE","DO THE RIGHT THING","THE GRADUATE","THE APARTMENT","RED RIVER","RAGING BULL","SINGIN' IN THE RAIN","BACK TO THE FUTURE","THE GODFATHER","SHERLOCK JR"].iter().copied().collect();
    let tv: Set = ["THE HONEYMOONERS","THE SUPER BOWL","PEE WEE'S PLAYHOUSE","THE WIRE","LEAVE IT TO BEAVER","THE CBS EVENING NEWS WITH WALTER CRONKITE","TWIN PEAKS","ST ELSEWHERE","DRAGNET","SOAP","THE ED SULLIVAN SHOW","HOMICIDE: LIFE ON THE STREET","THE BOB NEWHART SHOW","MOONLIGHTING","THE LARRY SANDERS SHOW","SECOND CITY TELEVISION","GUNSMOKE","HILL STREET BLUES","THE SINGING DETECTIVE","ALFRED HITCHCOCK PRESENTS","THE BEAVIS AND BUTT-HEAD SHOW","THE MONKEES","TAXI","THE DICK VAN DYKE SHOW","SOUTH PARK","ALL IN THE FAMILY","AN AMERICAN FAMILY","THE OFFICE","THE ERNIE KOVACS SHOW","SESAME STREET","SEE IT NOW","STAR TREK","THE FRENCH CHEF","MY SO-CALLED LIFE","WISEGUY","THE REAL WORLD","SURVIVOR","THE SOPRANOS","GILMORE GIRLS","THE X-FILES","THE DAY AFTER","FREAKS AND GEEKS","ARRESTED DEVELOPMENT","I, CLAUDIUS","SIX FEET UNDER","THE OPRAH WINFREY SHOW","WKRP IN CINCINNATI","I LOVE LUCY","THE ABBOTT AND COSTELLO SHOW","FRIENDS","MASH","LATE NIGHT WITH DAVID LETTERMAN","THE PRISONER","BUFFALO BILL","MONTY PYTHON'S FLYING CIRCUS","ROSEANNE","PLAYHOUSE 90","SPORTSCENTER","PRIME SUSPECT","THE GEORGE BURNS AND GRACIE ALLEN SHOW","MARRIED WITH CHILDREN","ROCKY AND HIS FRIENDS","GENERAL HOSPITAL","BUFFY THE VAMPIRE SLAYER","SEX AND THE CITY","BATTLESTAR GALACTICA","60 MINUTES","SPONGEBOB SQUAREPANTS","AMERICAN IDOL","THE PRICE IS RIGHT","SANFORD AND SON","THE SIMPSONS","WHAT'S MY LINE","LOST","THE COSBY SHOW","THE WEST WING","DEADWOOD","MARY HARTMAN, MARY HARTMAN","THE MARY TYLER MOORE SHOW","MYSTERY SCIENCE THEATER 3000","ABC'S WIDE WORLD OF SPORTS","THE SHIELD","24","ROOTS","THE TWILIGHT ZONE","SEINFELD","SATURDAY NIGHT LIVE","DALLAS","A CHARLIE BROWN CHRISTMAS","THE CAROL BURNETT SHOW","THE DAILY SHOW","KING OF THE HILL","FELICITY","BRIDESHEAD REVISITED","CHEERS","THE ODD COUPLE","THE TONIGHT SHOW STARRING JOHNNY CARSON"].iter().copied().collect();
    println!("{}", find_regex(&mut movies, &tv));
    let mut stars: Set = ["WILLIAM HOLDEN","LAURENCE OLIVIER","FRED ASTAIRE","JOHN WAYNE","JUDY GARLAND","BARBARA STANWYCK","AVA GARDNER","GRACE KELLY","EDWARD G. ROBINSON","ROBERT MITCHUM","MARY PICKFORD","SHIRLEY TEMPLE","CAROLE LOMBARD","KIRK DOUGLAS","CLAUDETTE COLBERT","SOPHIA LOREN","JAMES CAGNEY","MARLENE DIETRICH","CLARK GABLE","JEAN HARLOW","SIDNEY POITIER","MARX BROTHERS","INGRID BERGMAN","GREGORY PECK","HUMPHREY BOGART","HENRY FONDA","AUDREY HEPBURN","JAMES STEWART","BUSTER KEATON","ORSON WELLES","MAE WEST","VIVIEN LEIGH","GRETA GARBO","LAUREN BACALL","JAMES DEAN","CARY GRANT","SPENCER TRACY","JOAN CRAWFORD","BURT LANCASTER","ELIZABETH TAYLOR","GINGER ROGERS","BETTE DAVIS","CHARLIE CHAPLIN","MARLON BRANDO","MARILYN MONROE","GARY COOPER","GENE KELLY","LILLIAN GISH","KATHARINE HEPBURN","RITA HAYWORTH"].iter().copied().collect();
    let scientists: Set = ["ALAN GUTH","ANDREW KNOLL","MARGARET GELLER","C NUSSLEIN-VOLHARD","MILDRED DRESSELHAUS","ROBERT MARKS II","DENNIS BRAY","LENE VESTERGAARD HAU","EDWARD WILSON","ALAIN ASPECT","TIMOTHY BERNERS-LEE","JOHN TYLER BONNER","JANE GOODALL","CHARLES KAO","JACK SZOSTAK","SEIJI OGAWA","SYDNEY BRENNER","LEROY HOOD","JEAN FRECHET","KARY MULLIS","ERIC KANDEL","ANTHONY FIRE","GORDON MOORE","HAROLD VARMUS","CHARLES TOWNES","GERALD M EDELMAN","JAMES WATSON","EDWARD WITTEN","ANTHONY FAUCI","JAMES TOUR","ROGER PENROSE","HENRY F SCHAEFER III","PETER HIGGS","STEVEN WEINBERG","LUC MONTAGNIER","CRAIG MELLO","ALLEN BARD","GEORGE WHITESIDES","DAVID BALTIMORE","PIERRE CHAMBON","STEPHEN HAWKING","MARTIN KARPLUS","STANLEY PRUSINER","DONALD KNUTH","CRAIG VENTER","SHINYA YAMANAKA","THOMAS SUDHOF","JEREMIAH OSTRIKER","RONALD EVANS","SIMON CONWAY MORRIS"].iter().copied().collect();
    println!("{}", find_regex(&mut stars, &scientists));
    }
