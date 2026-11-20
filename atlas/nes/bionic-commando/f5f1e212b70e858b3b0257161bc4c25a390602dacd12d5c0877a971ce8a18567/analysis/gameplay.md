# Bionic Commando gameplay familiarization — round001

ROM title confirms Bionic Commando, Capcom1988, English Nintendo license title.
Guide applicability: likely US NES release; exact revision not yet established.
External descriptions are experiment hypotheses, not classification evidence.

Manual transcription https://www.world-of-nintendo.com/manuals/nes/bionic_commando.shtml
(version NES-CM-USA) describes map starting area0, travel/descend choice,
weapon selection and four communicator types. Transcription warns original manual
confuses A/B. GameFAQs guide search result
https://gamefaqs.gamespot.com/nes/589339-bionic-commando/faqs/7029
says B extends bionic arm; directional movement/swing, no jump. Need confirm in ROM.
StrategyWiki https://strategywiki.org/wiki/Bionic_Commando_%28NES%29/Walkthrough
starts area1; Area12 https://strategywiki.org/wiki/Bionic_Commando_%28NES%29/Area_12
leads after Albatross destruction to final stage. Final
https://strategywiki.org/wiki/Bionic_Commando_%28NES%29/Final_Area
requires shooting helicopter cockpit while falling, then ending/credits.

Data Crystal RAM/ROM map search discovered
https://datacrystal.tcrf.net/wiki/Bionic_Commando/RAM_map and
https://datacrystal.tcrf.net/wiki/Bionic_Commando/ROM_map but open returned403;
no RAM facts assumed from inaccessible sources. Local main dispatcher uses
$CB,$CC,$CD selector bytes (decoded evidence main-dispatch-decode.json), but
contract not yet established. Opening dialogue requires new A presses;
held A alone reached a completed text page with down triangle at frame1058.
A pulse/wait route in progress; trace disabled during long text waits and explicitly
reenabled afterward, so wait intervals are not execution coverage evidence.

Indexed search retrieved the Data Crystal ROM map despite direct403:
https://datacrystal.tcrf.net/wiki/Bionic_Commando_%28NES%29/ROM_map .
It places screen data$00610..$0600F, structure data$08810..$0EC0F and game
CHR$10010..$2800F (physical file offsets). These bounds corroborate locally
inspected repeated patterns and tile sheets; external claims alone are not map
annotations. Intro code exists near$07910 and executable code follows structure
bank at$0EC10, so entire low banks must not be broadly called data. Similarly,
metasprite-like banks20/21 have actual code at physical$2B08A (CPU$B07A).

Local shortcut: clean pre-shortcut snapshot frame3002; set area$4D=1,
room$4C=0, state$CC..CF=[3,0,0,0], reassert released input. This uses the
observed equipment menu's statically decoded outgoing continuation$AACB,
which advances$CC and clears its children. Area1 fully rendered at3075 after
initialization and right+A+B inputs. Saved area1-loading-snapshot.json/frame.
$CB remains1. Artificial route; normal map controls unresolved. B cancels the
equipment menu; A selects its current category. Do not repeat A waiting as if it
were guaranteed End confirmation. Later-area shortcuts are now tested from clean
snapshot with area/room pairs12/11,13/12 and12/96. Room96 is an explicit special
final-room choice in local map handler when area12 prerequisites are satisfied;
patching it bypasses those prerequisites and does not prove natural progression.
