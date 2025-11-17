restructure complete
but in the process i deleted all of my code
time do it all again
let's freaking go

have made good progress
give ai this and learn proc macros like a badass

```prompt
you know what this means??

rustadventofcodeprojectnamedrusty-aocwithworkspaceandmembersincludingmaincrateandaoc-macrosproc-macrocrate;maincratehasstructurewithsrc/bin/main.rsentrypointandmodulartreeundersrc/year_YYYY/day_NNcontaininginput.txtpart_01.rspart_02.rsget_input.rsetc;goaliscompile-timevalidationofadventofcodeinputfilestoeliminateruntimeparsingfailuresusingper-dayproceduralmacros;strategycreatesingleproc-macrocrateaoc-macroswithproc-macro=trueinCargo.tomlanddependenciesproc-macro2quoteandsynfull;implement#[aoc_day_input(path)]attributemacroinlib.rsthat(1)readsattributeasLitStrpath(2)readsfileatcompiletimeviastd::fs::read_to_stringwithpanicondiskerror(3)parsesinput_fnItemFntoextractfirstparameterstypeT(4)generatesconst__AOC_PARSED_INPUT:T=usinguser-providedparse_input(&str)->Tfunctioninsamemoduleviause crate::parse_inputandinclude_str!(path)(5)replacesfunctionbodytobindinput=&__AOC_PARSED_INPUTwhilepreservingoriginalblock(6)emitsbothconstandmodifiedfunction;userperdaywritesparse_input(&str)->CustomTypewithpanic!oninvaliddatae.g.s.lines().map(|l|l.parse::<i32>().unwrap()).collect::<Vec<i32>>();applies#[aoc_day_input("year_YYYY/day_NN/input.txt")]abovepub fn part_XX(input:&CustomType)->Answer{...};compile-timebehavior:filemissingorparse_inputpanicsresultsindetailedcompilererror;successemitsconstwithfullyparseddatanoallocationatruntime;functionreceives&'staticreferencetoconstdata;noextramacroargumentsneedednolinesizesinceparsinglogicencapsulatesallvalidation;parse_inputcanbeprivatemod__aoc_privateorpublic;macrofindsviacrate::parse_input;supportsanytypeVec<T>arraygridstructtupleetc;runtimecostzero;buildfailsoninputchangeviolatingparse_logic;testsviafunctioncallsintegrationtestsfailtocompileifinputinvalid;workspaceCargo.tomlhas[workspace]members=[".","aoc-macros"];eachdaydependsonaoc-macrosviapathorversion;exampleusage:mod__aoc_private{pubfnparse_input(s:&str)->Vec<i32>{s.lines().map(|l|l.trim().parse().unwrap_or_else(|e|panic!("badint{}:{}",l,e))).collect()}}use__aoc_private::parse_input;#[aoc_day_input("year_2015/day_01/input.txt")]pubfnpart_01(input:&[i32])->i32{input.iter().sum()};macroimplementationusesyn::parse_macro_input!forattrasLitStranditemasItemFnextractsfirstparamtypeviafn.sig.inputs.first()generatesconstwithquote!{const__AOC_PARSED_INPUT:#ty={usecrate::parse_input;parse_input(::std::include_str!(#path))}}rewritesblockas{letinput=&#const_name;#orig_block}combinesviaquote!{#const#fn};panicsinparse_inputbecomecompileerrorswithline/fileinfo;enablesfullcustomvalidationchecksumsgridboundsuniquekeysetcinsidetheparser;noexternallibsneededbeyondstd;worksacrossyearsdays;scalabletothousandsoflines;zerooverheadatruntime;compiletimecostlinearininputsize;idealforadventofcodewhereinputsareknownandstatic;canextendtomacro-generatedtestsorbenchmarksbutcorefocusisinputintegrityatbuildtime.

Let's talk about how to build proc macros for this purpose

please dont make any changes to files you nerd :-)
```