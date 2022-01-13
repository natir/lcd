var searchIndex = JSON.parse('{\
"lcd":{"doc":"Use kmer count to detect low coverage part of readme","t":[0,0,0,0,0,0,0,5,5,13,3,13,13,13,4,4,3,3,3,13,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,12,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,12,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,12,11,5,11,11,11,11,11,11,12,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,12,11,12,11,12,11,11,12,12,12,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,12,11,11,11,11,11,11,12,12,12,5,5,13,4,13,13,4,4,13,13,6,13,13,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,12,12,12,12,12,12,12,12,12,12,5,5,5,5,5,12,3,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11],"n":["clean","cli","detect","error","filter","io","read2gap","set_nb_threads","clean","Clean","Command","Detect","Filter","Json","OutputFormat","SubCommand","SubCommandClean","SubCommandDetect","SubCommandFilter","Text","augment_args","augment_args","augment_args","augment_args","augment_args_for_update","augment_args_for_update","augment_args_for_update","augment_args_for_update","augment_subcommands","augment_subcommands_for_update","borrow","borrow","borrow","borrow","borrow","borrow","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","buffer_length","buffer_length","clone","clone_into","deref","deref","deref","deref","deref","deref","deref_mut","deref_mut","deref_mut","deref_mut","deref_mut","deref_mut","drop","drop","drop","drop","drop","drop","fmt","fmt","fmt","fmt","fmt","fmt","format","format","from","from","from","from","from","from","from_arg_matches","from_arg_matches","from_arg_matches","from_arg_matches","from_arg_matches","from_str","from_subset","from_subset","from_subset","from_subset","from_subset","from_subset","gap_length","gap_length","has_subcommand","i82level","init","init","init","init","init","init","inputs","into","into","into","into","into","into","into_app","into_app","into_app","into_app","into_app","into_app_for_update","into_app_for_update","into_app_for_update","into_app_for_update","into_app_for_update","is_in_subset","is_in_subset","is_in_subset","is_in_subset","is_in_subset","is_in_subset","kmer_size","min_coverage","min_coverage","output","output","output_as_string","outputs","outputs","subcmd","threads","to_owned","to_subset","to_subset","to_subset","to_subset","to_subset","to_subset","to_subset_unchecked","to_subset_unchecked","to_subset_unchecked","to_subset_unchecked","to_subset_unchecked","to_subset_unchecked","try_from","try_from","try_from","try_from","try_from","try_from","try_into","try_into","try_into","try_into","try_into","try_into","type_id","type_id","type_id","type_id","type_id","type_id","update_from_arg_matches","update_from_arg_matches","update_from_arg_matches","update_from_arg_matches","update_from_arg_matches","verbosity","vzip","vzip","vzip","vzip","vzip","vzip","0","0","0","detect","main","CantOpenFile","Cli","Cli","DiffInputOutput","Error","IO","IO","OutputFormatCast","Result","Transparent","WriteError","borrow","borrow","borrow","borrow_mut","borrow_mut","borrow_mut","deref","deref","deref","deref_mut","deref_mut","deref_mut","drop","drop","drop","fmt","fmt","fmt","fmt","fmt","fmt","from","from","from","from","from","from","from_subset","from_subset","from_subset","init","init","init","into","into","into","is_in_subset","is_in_subset","is_in_subset","source","to_string","to_string","to_string","to_subset","to_subset","to_subset","to_subset_unchecked","to_subset_unchecked","to_subset_unchecked","try_from","try_from","try_from","try_into","try_into","try_into","type_id","type_id","type_id","vzip","vzip","vzip","input","output","params","0","0","0","error","error","path","path","filter","main","count_kmer","get_reader","get_writer","0","Read2Gap","borrow","borrow_mut","default","deref","deref_mut","deserialize","drop","fmt","from","from_subset","init","into","is_in_subset","json","serialize","text","to_subset","to_subset_unchecked","try_from","try_into","type_id","vzip"],"q":["lcd","","","","","","","","lcd::clean","lcd::cli","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","lcd::cli::SubCommand","","","lcd::detect","","lcd::error","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","lcd::error::Cli","","","lcd::error::Error","","","lcd::error::IO","","","","lcd::filter","","lcd::io","","","lcd::read2gap","","","","","","","","","","","","","","","","","","","","","","",""],"d":["Run cleaning of low coverage region inside of read","Command line interface of LCD","Run detection of low coverage region","LCD error enum’s","Run filtration of read with low coverage region or low …","Run detection of low coverage region","A collections to store low covered region associate to a …","Set the number of threads use by pcon","","","Low Coverage Detection","","","","","","Remove low coverage region","Detect low coverage region","Filter read with low coverage region","","","","","","","","","","","","","","","","","","","","","","","","Get buffer_length or default value","Number of sequence record load in buffer (default: 8192)","","","","","","","","","","","","","","","","","","","","","","","","","","","Get format or default value","Output format (default: text)","","","","","","","","","","","","","","","","","","","Get gap_length or default value","Minimal length of coverage gap to be consider as an …","","Convert verbosity count to log Level","","","","","","","Path to inputs","","","","","","","","","","","","","","","","","","","","","","","Size of kmer","Get min_coverage or default value","If estimate coverage is equal or lower than this value are …","Get output or default value","Output path (default: stdout)","Get output or default value as String","Get outputs or default value","Output path (default: stdout)","","Number of thread use by pcon to count, 0 use all avaible …","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","verbosity level also control by environment variable …","","","","","","","","","","Run detection on input files with parameter","Run detection with parameter from cli","Can’t open file.","","Error associate to Command Line Interface Cli","Number of input and output must be equal","","","Input Output error IO","Can’t convert str in OutputFormat","","","Write error","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","Filter sequence in inputs based on read2gap information …","Run filter with parameter from cli","","","","","Struct to associate a read id his gap","","","","","","","","","","","","","","Convert Read2Gap instance in json format","","Convert Read2Gap instance in human text format","","","","","",""],"i":[0,0,0,0,0,0,0,0,0,1,0,1,1,2,0,0,0,0,0,2,3,4,5,6,3,4,5,6,1,1,3,1,2,4,5,6,3,1,2,4,5,6,3,3,2,2,3,1,2,4,5,6,3,1,2,4,5,6,3,1,2,4,5,6,3,1,2,4,5,6,4,4,3,1,2,4,5,6,3,1,4,5,6,2,3,1,2,4,5,6,3,3,1,0,3,1,2,4,5,6,3,3,1,2,4,5,6,3,1,4,5,6,3,1,4,5,6,3,1,2,4,5,6,3,3,3,4,4,4,5,5,3,3,2,3,1,2,4,5,6,3,1,2,4,5,6,3,1,2,4,5,6,3,1,2,4,5,6,3,1,2,4,5,6,3,1,4,5,6,3,3,1,2,4,5,6,7,8,9,0,0,10,0,11,12,0,0,11,12,0,11,10,11,12,10,11,12,10,11,12,10,11,12,10,11,12,10,11,11,12,12,10,10,11,11,11,11,12,10,11,12,10,11,12,10,11,12,10,11,12,10,11,11,12,10,11,12,10,11,12,10,11,12,10,11,12,10,11,12,10,11,12,10,13,13,14,15,16,17,18,19,18,19,0,0,0,0,0,20,0,20,20,20,20,20,20,20,20,20,20,20,20,20,20,20,20,20,20,20,20,20,20],"f":[null,null,null,null,null,null,null,[[["usize",15]]],[[["subcommandclean",3]],["result",6]],null,null,null,null,null,null,null,null,null,null,null,[[["app",3]],["app",3]],[[["app",3]],["app",3]],[[["app",3]],["app",3]],[[["app",3]],["app",3]],[[["app",3]],["app",3]],[[["app",3]],["app",3]],[[["app",3]],["app",3]],[[["app",3]],["app",3]],[[["app",3]],["app",3]],[[["app",3]],["app",3]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[],["usize",15]],null,[[],["outputformat",4]],[[]],[[["usize",15]]],[[["usize",15]]],[[["usize",15]]],[[["usize",15]]],[[["usize",15]]],[[["usize",15]]],[[["usize",15]]],[[["usize",15]]],[[["usize",15]]],[[["usize",15]]],[[["usize",15]]],[[["usize",15]]],[[["usize",15]]],[[["usize",15]]],[[["usize",15]]],[[["usize",15]]],[[["usize",15]]],[[["usize",15]]],[[["formatter",3]],["result",6]],[[["formatter",3]],["result",6]],[[["formatter",3]],["result",6]],[[["formatter",3]],["result",6]],[[["formatter",3]],["result",6]],[[["formatter",3]],["result",6]],[[],["outputformat",4]],null,[[]],[[]],[[]],[[]],[[]],[[]],[[["argmatches",3]],[["result",4,["error"]],["error",3]]],[[["argmatches",3]],[["result",4,["error"]],["error",3]]],[[["argmatches",3]],[["result",4,["error"]],["error",3]]],[[["argmatches",3]],[["result",4,["error"]],["error",3]]],[[["argmatches",3]],[["result",4,["error"]],["error",3]]],[[["str",15]],["result",4]],[[]],[[]],[[]],[[]],[[]],[[]],[[],["u8",15]],null,[[["str",15]],["bool",15]],[[["i8",15]],[["option",4,["level"]],["level",4]]],[[],["usize",15]],[[],["usize",15]],[[],["usize",15]],[[],["usize",15]],[[],["usize",15]],[[],["usize",15]],null,[[]],[[]],[[]],[[]],[[]],[[]],[[],["app",3]],[[],["app",3]],[[],["app",3]],[[],["app",3]],[[],["app",3]],[[],["app",3]],[[],["app",3]],[[],["app",3]],[[],["app",3]],[[],["app",3]],[[],["bool",15]],[[],["bool",15]],[[],["bool",15]],[[],["bool",15]],[[],["bool",15]],[[],["bool",15]],null,[[],["u8",15]],null,[[],[["box",3,["write"]],["result",6,["box"]]]],null,[[],["string",3]],[[],[["result",6,["vec"]],["vec",3,["box"]]]],null,null,null,[[]],[[],["option",4]],[[],["option",4]],[[],["option",4]],[[],["option",4]],[[],["option",4]],[[],["option",4]],[[]],[[]],[[]],[[]],[[]],[[]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["typeid",3]],[[],["typeid",3]],[[],["typeid",3]],[[],["typeid",3]],[[],["typeid",3]],[[],["typeid",3]],[[["argmatches",3]],[["result",4,["error"]],["error",3]]],[[["argmatches",3]],[["result",4,["error"]],["error",3]]],[[["argmatches",3]],[["result",4,["error"]],["error",3]]],[[["argmatches",3]],[["result",4,["error"]],["error",3]]],[[["argmatches",3]],[["result",4,["error"]],["error",3]]],null,[[]],[[]],[[]],[[]],[[]],[[]],null,null,null,[[["usize",15],["counter",3],["u8",15]],[["read2gap",3],["result",6,["read2gap"]]]],[[["subcommanddetect",3],["counter",3],["command",3]],["result",6]],null,null,null,null,null,null,null,null,null,null,null,[[]],[[]],[[]],[[]],[[]],[[]],[[["usize",15]]],[[["usize",15]]],[[["usize",15]]],[[["usize",15]]],[[["usize",15]]],[[["usize",15]]],[[["usize",15]]],[[["usize",15]]],[[["usize",15]]],[[["formatter",3]],["result",6]],[[["formatter",3]],["result",6]],[[["formatter",3]],["result",6]],[[["formatter",3]],["result",6]],[[["formatter",3]],["result",6]],[[["formatter",3]],["result",6]],[[["io",4]]],[[]],[[["box",3,["error"]],["error",8]]],[[["cli",4]]],[[]],[[]],[[]],[[]],[[]],[[],["usize",15]],[[],["usize",15]],[[],["usize",15]],[[]],[[]],[[]],[[],["bool",15]],[[],["bool",15]],[[],["bool",15]],[[],[["option",4,["error"]],["error",8]]],[[],["string",3]],[[],["string",3]],[[],["string",3]],[[],["option",4]],[[],["option",4]],[[],["option",4]],[[]],[[]],[[]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["typeid",3]],[[],["typeid",3]],[[],["typeid",3]],[[]],[[]],[[]],null,null,null,null,null,null,null,null,null,null,[[["usize",15],["read2gap",3],["box",3,["write"]],["vec",3,["box"]]],["result",6]],[[["subcommandfilter",3],["counter",3],["command",3]],["result",6]],[[["usize",15],["u8",15]],[["result",6,["counter"]],["counter",3]]],[[["path",3]],[["box",3,["read"]],["result",6,["box"]]]],[[["path",3]],[["box",3,["write"]],["result",6,["box"]]]],null,null,[[]],[[]],[[],["read2gap",3]],[[["usize",15]]],[[["usize",15]]],[[],["result",4]],[[["usize",15]]],[[["formatter",3]],["result",6]],[[]],[[]],[[],["usize",15]],[[]],[[],["bool",15]],[[],[["result",6,["string"]],["string",3]]],[[],["result",4]],[[],[["result",6,["string"]],["string",3]]],[[],["option",4]],[[]],[[],["result",4]],[[],["result",4]],[[],["typeid",3]],[[]]],"p":[[4,"SubCommand"],[4,"OutputFormat"],[3,"Command"],[3,"SubCommandDetect"],[3,"SubCommandFilter"],[3,"SubCommandClean"],[13,"Detect"],[13,"Filter"],[13,"Clean"],[4,"IO"],[4,"Error"],[4,"Cli"],[13,"DiffInputOutput"],[13,"OutputFormatCast"],[13,"IO"],[13,"Cli"],[13,"Transparent"],[13,"CantOpenFile"],[13,"WriteError"],[3,"Read2Gap"]]}\
}');
if (window.initSearch) {window.initSearch(searchIndex)};