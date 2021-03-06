# 密码学原理与实践

## 1.5

移位密码破解 BEEAKFYDJXUQYHYJIQRYHTYJIQFBQDUYJIIKFUHCQD:

枚举移位：

```
1: CFFBLGZEKYVRZIZKJRSZIUZKJRGCREVZKJJLGVIDRE
2: DGGCMHAFLZWSAJALKSTAJVALKSHDSFWALKKMHWJESF
3: EHHDNIBGMAXTBKBMLTUBKWBMLTIETGXBMLLNIXKFTG
4: FIIEOJCHNBYUCLCNMUVCLXCNMUJFUHYCNMMOJYLGUH
5: GJJFPKDIOCZVDMDONVWDMYDONVKGVIZDONNPKZMHVI
6: HKKGQLEJPDAWENEPOWXENZEPOWLHWJAEPOOQLANIWJ
7: ILLHRMFKQEBXFOFQPXYFOAFQPXMIXKBFQPPRMBOJXK
8: JMMISNGLRFCYGPGRQYZGPBGRQYNJYLCGRQQSNCPKYL
9: KNNJTOHMSGDZHQHSRZAHQCHSRZOKZMDHSRRTODQLZM
10: LOOKUPINTHEAIRITSABIRDITSAPLANEITSSUPERMAN
```

结果： Look up in the air. It's a bird. It's a plane. It's superman.

## 1.16

a)

原置换：

| x     | 1    | 2    | 3    | 4    | 5    | 6    | 7    | 8    |
| ----- | ---- | ---- | ---- | ---- | ---- | ---- | ---- | ---- |
| pi(x) | 4    | 1    | 6    | 2    | 7    | 3    | 8    | 5    |

逆：

| pi(x) | 1    | 2    | 3    | 4    | 5    | 6    | 7    | 8    |
| ----- | ---- | ---- | ---- | ---- | ---- | ---- | ---- | ---- |
| x     | 2    | 4    | 6    | 1    | 8    | 3    | 5    | 7    |

b)

ETEGENLM DNTNEOOR DAHATECO ESAHLRMI

变为

GENTLEME NDONOTRE ADEACHOT HERSMAIL

Gentlemen do not read each others mail.

## 1.21

a)

代换密码

EMGLOSUDCGDNCUSWYSFHNSFCYKDPUMLWGYICOXYSIPJCKQPKUGKMGOLICGINCGACKSNISACYKZSCKXECJCKSHYSXCGOIDPKZCNKSHICGIWYGKKGKGOLDSILKGOIUSIGLEDSPWZUGFZCCNDGYYSFUSZCNXEOJNCGYEOWEUPXEZGACGNFGLKNSACIGOIYCKXCJUCIUZCFZCCNDGYYSFEUEKUZCSOCFZCCNCIACZEJNCSHFZEJZEGMXCYHCJUMGKUCY

出现次数最多：C（14.45%），G（9.38%），S（7.81%），K（7.03%）

于是猜测 C -> e，得到

EMGLOSUDeGDNeUSWYSFHNSFeYKDPUMLWGYIeOXYSIPJeKQPKUGKMGOLIeGINeGAeKSNISAeYKZSeKXEeJeKSHYSXeGOIDPKZeNKSHIeGIWYGKKGKGOLDSILKGOIUSIGLEDSPWZUGFZeeNDGYYSFUSZeNXEOJNeGYEOWEUPXEZGAeGNFGLKNSAeIGOIYeKXeJUeIUZeFZeeNDGYYSFEUEKUZeSOeFZeeNeIAeZEJNeSHFZEJZEGMXeYHeJUMGKUeY

Ie eI 出现次数比较多，猜测 I -> d，得到

EMGLOSUDeGDNeUSWYSFHNSFeYKDPUMLWGYdeOXYSdPJeKQPKUGKMGOLdeGdNeGAeKSNdSAeYKZSeKXEeJeKSHYSXeGOdDPKZeNKSHdeGdWYGKKGKGOLDSdLKGOdUSdGLEDSPWZUGFZeeNDGYYSFUSZeNXEOJNeGYEOWEUPXEZGAeGNFGLKNSAedGOdYeKXeJUedUZeFZeeNDGYYSFEUEKUZeSOeFZeeNedAeZEJNeSHFZEJZEGMXeYHeJUMGKUeY

eK 出现次数较多，但没有 Ke，猜测 K -> s，得到

EMGLOSUDeGDNeUSWYSFHNSFeYsDPUMLWGYdeOXYSdPJesQPsUGsMGOLdeGdNeGAesSNdSAeYsZSesXEeJesSHYSXeGOdDPsZeNsSHdeGdWYGssGsGOLDSdLsGOdUSdGLEDSPWZUGFZeeNDGYYSFUSZeNXEOJNeGYEOWEUPXEZGAeGNFGLsNSAedGOdYesXeJUedUZeFZeeNDGYYSFEUEsUZeSOeFZeeNedAeZEJNeSHFZEJZEGMXeYHeJUMGsUeY

G 出现次数多，猜测 G -> a，得到


EMaLOSUDeaDNeUSWYSFHNSFeYsDPUMLWaYdeOXYSdPJesQPsUasMaOLdeadNeaAesSNdSAeYsZSesXEeJesSHYSXeaOdDPsZeNsSHdeadWYassasaOLDSdLsaOdUSdaLEDSPWZUaFZeeNDaYYSFUSZeNXEOJNeaYEOWEUPXEZaAeaNFaLsNSAedaOdYesXeJUedUZeFZeeNDaYYSFEUEsUZeSOeFZeeNedAeZEJNeSHFZEJZEaMXeYHeJUMasUeY

aOd 出现次数较多，猜测 O -> n，得到

EMaLnSUDeaDNeUSWYSFHNSFeYsDPUMLWaYdenXYSdPJesQPsUasManLdeadNeaAesSNdSAeYsZSesXEeJesSHYSXeandDPsZeNsSHdeadWYassasanLDSdLsandUSdaLEDSPWZUaFZeeNDaYYSFUSZeNXEnJNeaYEnWEUPXEZaAeaNFaLsNSAedandYesXeJUedUZeFZeeNDaYYSFEUEsUZeSneFZeeNedAeZEJNeSHFZEJZEaMXeYHeJUMasUeY

其余里面 S 出现次数较多（7.81%），按照字母频率，猜测 S -> o，得到

EMaLnoUDeaDNeUoWYoFHNoFeYsDPUMLWaYdenXYodPJesQPsUasManLdeadNeaAesoNdoAeYsZoesXEeJesoHYoXeandDPsZeNsoHdeadWYassasanLDodLsandUodaLEDoPWZUaFZeeNDaYYoFUoZeNXEnJNeaYEnWEUPXEZaAeaNFaLsNoAedandYesXeJUedUZeFZeeNDaYYoFEUEsUZeoneFZeeNedAeZEJNeoHFZEJZEaMXeYHeJUMasUeY

U 出现 5.47%，按照字母频率和 noU Uo 猜测 U -> t，得到


EMaLnotDeaDNetoWYoFHNoFeYsDPtMLWaYdenXYodPJesQPstasManLdeadNeaAesoNdoAeYsZoesXEeJesoHYoXeandDPsZeNsoHdeadWYassasanLDodLsandtodaLEDoPWZtaFZeeNDaYYoFtoZeNXEnJNeaYEnWEtPXEZaAeaNFaLsNoAedandYesXeJtedtZeFZeeNDaYYoFEtEstZeoneFZeeNedAeZEJNeoHFZEJZEaMXeYHeJtMasteY

从 ManL MasteY todaL 猜测 M -> m，L -> y, Y -> r，得到

EmaynotDeaDNetoWroFHNoFersDPtmyWardenXrodPJesQPstasmanydeadNeaAesoNdoAersZoesXEeJesoHroXeandDPsZeNsoHdeadWrassasanyDodysandtodayEDoPWZtaFZeeNDarroFtoZeNXEnJNearEnWEtPXEZaAeaNFaysNoAedandresXeJtedtZeFZeeNDarroFEtEstZeoneFZeeNedAeZEJNeoHFZEJZEamXerHeJtmaster

从 E may not 和 Eam 猜测 E -> i ，得到


imaynotDeaDNetoWroFHNoFersDPtmyWardenXrodPJesQPstasmanydeadNeaAesoNdoAersZoesXieJesoHroXeandDPsZeNsoHdeadWrassasanyDodysandtodayiDoPWZtaFZeeNDarroFtoZeNXinJNearinWitPXiZaAeaNFaysNoAedandresXeJtedtZeFZeeNDarroFitistZeoneFZeeNedAeZiJNeoHFZiJZiamXerHeJtmaster

从 myWarden 和 Wrass 猜测 W -> g，得到


imaynotDeaDNetogroFHNoFersDPtmygardenXrodPJesQPstasmanydeadNeaAesoNdoAersZoesXieJesoHroXeandDPsZeNsoHdeadgrassasanyDodysandtodayiDoPgZtaFZeeNDarroFtoZeNXinJNearingitPXiZaAeaNFaysNoAedandresXeJtedtZeFZeeNDarroFitistZeoneFZeeNedAeZiJNeoHFZiJZiamXerHeJtmaster

从 De 和 anyDody 猜测 D -> b，得到


imaynotbeabNetogroFHNoFersbPtmygardenXrodPJesQPstasmanydeadNeaAesoNdoAersZoesXieJesoHroXeandbPsZeNsoHdeadgrassasanybodysandtodayiboPgZtaFZeeNbarroFtoZeNXinJNearingitPXiZaAeaNFaysNoAedandresXeJtedtZeFZeeNbarroFitistZeoneFZeeNedAeZiJNeoHFZiJZiamXerHeJtmaster

从 abNe 猜测 N -> l，得到

imaynotbeabletogroFHloFersbPtmygardenXrodPJesQPstasmanydeadleaAesoldoAersZoesXieJesoHroXeandbPsZelsoHdeadgrassasanybodysandtodayiboPgZtaFZeelbarroFtoZelXinJlearingitPXiZaAealFaysloAedandresXeJtedtZeFZeelbarroFitistZeoneFZeeledAeZiJleoHFZiJZiamXerHeJtmaster

从 bPt 和 boPgZt 猜测 P -> u，Z -> h，得到

imaynotbeabletogroFHloFersbutmygardenXroduJesQustasmanydeadleaAesoldoAershoesXieJesoHroXeandbushelsoHdeadgrassasanybodysandtodayiboughtaFheelbarroFtohelXinJlearingituXihaAealFaysloAedandresXeJtedtheFheelbarroFitistheoneFheeledAehiJleoHFhiJhiamXerHeJtmaster

由 F -> w，可以推断并找到原文是：

I may not be able to grow flowers but my garden produces just as many dead leaves, old overshoes, pieces of rope and bushels of dead grass as anybody's, and today I bought a wheelbarrow to help in clearing it up. I have always loved and respected the wheelbarrow. It is the one wheeled vehicle of which I am perfect master.

b)

维吉尼亚密码

KCCPKBGUFDPHQTYAVINRRTMVGRKDNBVFDETDGILTXRGUDDKOTFMBPVGEGLTGCKQRACQCWDNAWCRXIZAKFTLEWRPTYCQKYVXCHKFTPONCQQRHJVAJUWETMCMSPKQDYHJVDAHCTRLSVSKCGCZQQDZXGSFRLSWCWSJTBHAFSIASPRJAHKJRJUMVGKMITZHFPDISPZLVLGWTFPLKKEBDPGCEBSHCTJRWXBAFSPEZQNRWXCVYCGAONWDDKACKAWBBIKFTIOVKCGGHJVLNHIFFSQESVYCLACNVRWBBIREPBBVFEXOSCDYGZWPFDTKFQIYCWHJVLNHIQIBTKHJVNPIST

HJV 出现 5 次，分别在 107 125 263 317 329 位置，差值是 18 138 54 12，得到最大公约数是 6 。

拆分得到以下六个字符串：

KGQNGVGGTGCQWAWQHNJEPJTKQFWAPJGHPWKCTAQVNCIVJFVNIVCPQJIVT
CUTRRFIUFEKCCKRKKCVTKVRCDRSFRRKFZTEEJFNYWKKKVFYVRFDFIVIV
CFYRKDLDMGQWRFPYFQAMQDLGZLJSJJMPLFBBRSRCDAFCLSCREEYDYLBN
PDATDETDBLRDXTTVTQJCDASCXSTIAUIDVPDSWPWGDWTGNQLWPXGTCNTP
KPVMNTXKPTANILYXPRUMYHVZGWBAHMTILLPHXEXAKBIGHEABBOZKWHKI
BHIVBDROVGCAZECCOHWSHCSQSCHSKVZSGKGCBZCOABOHISCBBSWFHIHS

按照六个字符串，分别寻找 Mg 接近 0.065 的，得到密钥是 CRYPTO

对上面六个字符串分别进行解密：

IEOLETEEREAOUYUOFLHCNHRIODUYNHEFNUIARYOTLAGTHDTLGTANOHGTR
LDCAAORDONTLLTATTLECTEALMABOAATOICNNSOWHFTTTEOHEAOMORERE
EHATMFNFOISYTHRAHSCOSFNIBNLULLORNHDDTUTEFCHENUETGGAFANDP
AOLEOPEOMWCOIEEGEBUNOLDNIDETLFTOGAODHAHROHERYBWHAIRENYEA
RWCTUAERWAHUPSFEWYBTFOCGNDIHOTAPSSWOELEHRIPNOLHIIVGRDORP
NTUHNPDAHSOMLQOOATIETOECEOTEWHLESWSONLOAMNATUEONNEIRTUTE

再按照竖着的顺序写：

I learned how to calculate the amount of paper needed for a room when i was at school.
You multiply the square footage of the walls by the cubic contents of the floor and ceiling combined, and double it. You then allow half the total for openings such as windows and doors. Then you allow the other half for matching the pattern. Then you double the whole thing again to give a margin of error, and then you order the paper.

c)

仿射密码

KQEREJEBCPPCJCRKIEACUZBKRVPKRBCIBQCARBJCVFCUPKRIOFKPACUZQEPBKRXPEIIEABDKPBCPFCDCCAFIEABDKPBCPFEQPKAZBKRHAIBKAPCCIBURCCDKDCCJCIDFUIXPAFFERBICZDFKABICBBENEFCUPJCVKABPCYDCCDPKBCOCPERKIVKSCPICBRKIJPKABI

统计字母频率：C(16.16%) B(10.61%) K(10.10%) P(10.10%)

猜测：C -> e, B -> a

2 = k*4+b (mod 26)
1 = k*0+b (mod 26)

无解

猜测：C -> e, B -> i

2 = k*4+b (mod 26)
1 = k*8+b (mod 26)

无解

猜测：C -> e, B -> o

2 = k*4+b (mod 26)
1 = k*14+b (mod 26)

无解

猜测：C -> e, B -> t

2 = k*4+b (mod 26)
1 = k*19+b (mod 26)

b = 4, k = 19, k^{-1} = 11

此时 K -> o ，词频也比较高

按照 k=19, b=4 解出来：

OCANADATERREDENOSAIEUXTONFRONTESTCEINTDEFLEURONSBLORIEUXCARTONGRASSAITPORTERLEPEEILSAITPORTERLACROIXTONHISTOIREESTUNEEPOPEEDESPLUSGRILLANTSEXPLOITSETTAVALEURDEFOITREMPEEPROTEBERANOSFOYERSETNOSDROITS

发现是加拿大的国歌：

Ô Canada!
Terre de nos aïeux,
Ton front est ceint de fleurons glorieux!
Car ton bras sait porter l'épée,
Il sait porter la croix!
Ton histoire est une épopée
Des plus brillants exploits.
Et ta valeur, de foi trempée,
Protégera nos foyers et nos droits.

d) 未知密码

BNVSNSIHQCEELSSKKYERIFJKXUMBGYKAMQLJTYAVFBKVTDVBPVVRJYYLAOKYMPQSCGDLFSRLLPROYGESEBUUALRWXMMASAZLGLEDFJBZAVVPXWICGJXASCBYEHOSNMULKCEAHTQOKMFLEBKFXLRRFDTZXCIWBJSICBGAWDVYDHAVFJXZIBKCGJIWEAHTTOEWTUHKRQVVRGZBXYIREMMASCSPBHLHJMBLRFFJELHWEYLWISTFVVYEJCMHYUYRUFSFMGESIGRLWALSWMNUHSIMYYITCCQPZSICEHBCCMZFEGVJYOCDEMMPGHVAAUMELCMOEHVLTIPSUYILVGFLMVWDVYDBTHFRAYISYSGKVSUUHYHGGCKTMBLRX

统计一下词频，发现没有频率特别突出的字母。排除单表代换密码，考虑维吉尼亚密码。

发现 VVR 、 MMA 、 SIC 、 MAS 、 DVY 、 VYD 各出现了两次，位置的差分别是 149 120 127 120 174 174

DVYD 出现两次，位置的差是 174

所以，取 120 和 174 的最大公约数 6 。把原文按照六的周期拆分：

BILEXKTKPYMDLEAMGBXXEUHFXTBGDXGHTVXMBBELVMUEWNYQEZYMAMTIMDAGHKX
NHSRUAYVVLPLPSLALZWAHLTLLZJAHZJTUVYAHLLWVHFSAUYPHFOPUOILVBYKYT
VQSIMMATVAQFRERSEAISOKQERXSWAIITHRISLRHIYYSILHIZBECGMEPVWTIVHM
SCKFBQVDROSSOBWADVCCSCOBRCIDVBWOKGRCHFWSEUFGSSTSCGDHEHSGDHSSGB
NEKJGLFVJKCRYUXZFVGBNEKKFICVFKEERZESJFETJYMRWICICVEVLVUFVFYUGL
SEYKYJBBYYGLGUMLJPJYMAMFDWBYJCAWQBMPMJYFCRGLMMCCMJMACLYLYRSUCR

按照六个字符串，分别寻找 Mg 接近 0.065 的，得到密钥是 THEORY

得到对应的原文：

IPSLERARWFTKSLHTNIEELBOMEAINKENOACETIILSCTBLDUFXLGFTHTAPTKHNORE
GALKNTROOEIEILETESPTAEMEESCTASCMNORTAEEPOAYLTNRIAYHINHBEOURDRM
RMOEIIWPRWMBNANOAWEOKGMANTOSWEEPDNEOHNDEUUOEHDEVXAYCIALRSPERDI
EOWRNCHPDAEEANIMPHOOEOANDOUPHNIAWSDOTRIEQGRSEEFEOSPTQTESPTEESN
WNTSPUOESTLAHDGIOEPKWNTTORLEOTNNAINBSONCSHVAFRLRLENEUEDOEOHDPU
UGAMALDDAAINIWONLRLAOCOHFYDALECYSDOROLAHETINOOEEOLOCENANATUWET

化成正常顺序，搜索到原文：

I grew up among slow talkers, men in particular who dropped words a few at a time like beans in a hill, and when I got to Minneapolis where people took a Lake Wobegon comma to mean the end of a story, i couldn't speak a whole sentence in company and was considered not too bright. So i enrolled in a speech course taught by Orville Sand, the founder of reflexive relax ology, a self-hypnotic technique that enabled a person to speak up to three hundred words per minute.

## 1.26

a)

已知 m=4,n=3 的时候，Bob 只需要把 ctaropyghpry 按照矩阵的形式重新排列，然后换个方向读出来就行原文了。

b)

MYAMRARUYIQTENCTORAHROYWDSOYEOUARRGDERNOGW

长度是42，猜测是 6x7 、 7x6 、14x3 、 3x14。

如果是 6x7：

MREADUE

看起来并不正确。如果是 7x6:

MUCOED

看起来也不正确。如果是 14x3

MCE
YTO

如果是 3x14:

MMRIETAOD

也不对。

搜了下答案，感觉答案里的解密方法和上面例子里的解密方法并不一样。