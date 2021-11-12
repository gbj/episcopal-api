use crate::Book;

pub const BOOKS: [(&'static str, Book); 162] = [
    ("Genesis", Book::Genesis),
    ("Gen.", Book::Genesis),
    ("Exodus", Book::Exodus),
    ("Exod.", Book::Exodus),
    ("Ex.", Book::Exodus),
    ("Leviticus", Book::Leviticus),
    ("Lev.", Book::Leviticus),
    ("Numbers", Book::Numbers),
    ("Num.", Book::Numbers),
    ("Deuteronomy", Book::Deuteronomy),
    ("Deut.", Book::Deuteronomy),
    ("Joshua", Book::Joshua),
    ("Josh.", Book::Joshua),
    ("Judges", Book::Judges),
    ("Judg.", Book::Judges),
    ("Jg.", Book::Judges),
    ("Ruth", Book::Ruth),
    ("First Samuel", Book::FirstSamuel),
    ("Second Samuel", Book::SecondSamuel),
    ("First Kings", Book::FirstKings),
    ("Second Kings", Book::SecondKings),
    ("First Chronicles", Book::FirstChronicles),
    ("Second Chronicles", Book::SecondChronicles),
    ("1 Samuel", Book::FirstSamuel),
    ("2 Samuel", Book::SecondSamuel),
    ("1 Sam.", Book::FirstSamuel),
    ("2 Sam.", Book::SecondSamuel),
    ("1 Kings", Book::FirstKings),
    ("2 Kings", Book::SecondKings),
    ("1 Kgs", Book::FirstKings),
    ("2 Kgs", Book::SecondKings),
    ("1 Chronicles", Book::FirstChronicles),
    ("1 Chron.", Book::FirstChronicles),
    ("2 Chronicles", Book::SecondChronicles),
    ("2 Chron.", Book::SecondChronicles),
    ("Ezra", Book::Ezra),
    ("Neh.", Book::Nehemiah),
    ("Nehemiah", Book::Nehemiah),
    ("Est", Book::Esther),
    ("Esther", Book::Esther),
    ("Job", Book::Job),
    ("Psalms", Book::Psalms),
    ("Ps.", Book::Psalms),
    ("Proverbs", Book::Proverbs),
    ("Prov.", Book::Proverbs),
    ("Ecclesiastes", Book::Ecclesiastes),
    ("Eccl.", Book::Ecclesiastes),
    ("Song of Solomon", Book::SongOfSolomon),
    ("Song of Songs", Book::SongOfSolomon),
    ("Cantic", Book::SongOfSolomon),
    ("Isaiah", Book::Isaiah),
    ("Isa.", Book::Isaiah),
    ("Jeremiah", Book::Jeremiah),
    ("Jer.", Book::Jeremiah),
    ("Lamentations", Book::Lamentations),
    ("Lam.", Book::Lamentations),
    ("Ezekiel", Book::Ezekiel),
    ("Ezek.", Book::Ezekiel),
    ("Daniel", Book::Daniel),
    ("Dan.", Book::Daniel),
    ("Hosea", Book::Hosea),
    ("Joel", Book::Joel),
    ("Amos", Book::Amos),
    ("Obadiah", Book::Obadiah),
    ("Jonah", Book::Jonah),
    ("Micah", Book::Micah),
    ("Nahum", Book::Nahum),
    ("Habakkuk", Book::Habakkuk),
    ("Zephaniah", Book::Zephaniah),
    ("Zeph.", Book::Zephaniah),
    ("Haggai", Book::Haggai),
    ("Zechariah", Book::Zechariah),
    ("Zech.", Book::Zechariah),
    ("Malachi", Book::Malachi),
    ("Matthew", Book::Matthew),
    ("Matt.", Book::Matthew),
    ("Mark", Book::Mark),
    ("Luke", Book::Luke),
    ("John", Book::John),
    ("Acts", Book::Acts),
    ("Romans", Book::Romans),
    ("Rom.", Book::Romans),
    ("1 Cor.", Book::FirstCorinthians),
    ("1 Corinthians", Book::FirstCorinthians),
    ("First Corinthians", Book::FirstCorinthians),
    ("2 Cor.", Book::SecondCorinthians),
    ("2 Corinthians", Book::SecondCorinthians),
    ("Second Corinthians", Book::SecondCorinthians),
    ("Galatians", Book::Galatians),
    ("Gal.", Book::Galatians),
    ("Ephesians", Book::Ephesians),
    ("Eph", Book::Ephesians),
    ("Philippians", Book::Philippians),
    ("Phil.", Book::Philippians),
    ("Colossians", Book::Colossians),
    ("Col.", Book::Colossians),
    ("1 Thessalonians", Book::FirstThessalonians),
    ("First Thessalonians", Book::FirstThessalonians),
    ("1 Thess.", Book::FirstThessalonians),
    ("2 Thessalonians", Book::SecondThessalonians),
    ("2 Thess.", Book::SecondThessalonians),
    ("Second Thessalonians", Book::SecondThessalonians),
    ("1 Timothy", Book::FirstTimothy),
    ("First Timothy", Book::FirstTimothy),
    ("1 Tim", Book::FirstTimothy),
    ("2 Timothy", Book::SecondTimothy),
    ("Second Timothy", Book::SecondTimothy),
    ("2 Tim", Book::SecondTimothy),
    ("Titus", Book::Titus),
    ("Philemon", Book::Philemon),
    ("Phlm", Book::Philemon),
    ("Hebrews", Book::Hebrews),
    ("Heb.", Book::Hebrews),
    ("James", Book::James),
    ("1 Peter", Book::FirstPeter),
    ("First Peter", Book::FirstPeter),
    ("2 Peter", Book::SecondPeter),
    ("Second Peter", Book::SecondPeter),
    ("1 John", Book::FirstJohn),
    ("2 John", Book::SecondJohn),
    ("3 John", Book::ThirdJohn),
    ("First John", Book::FirstJohn),
    ("Second John", Book::SecondJohn),
    ("Third John", Book::ThirdJohn),
    ("Jude", Book::Jude),
    ("Revelation", Book::Revelation),
    ("Rev", Book::Revelation),
    ("Apocalypse", Book::Revelation),
    ("Ester", Book::Ester),
    ("Wisdom", Book::Wisdom),
    ("Wis. Sol.", Book::Wisdom),
    ("Wisdom of Solomon", Book::Wisdom),
    ("Ecclesiasticus", Book::Ecclesiasticus),
    ("Ecclus.", Book::Ecclesiasticus),
    ("Baruch", Book::Baruch),
    ("Epistle of Jeremiah", Book::EpistleJeremiah),
    ("Ep. Jer.", Book::EpistleJeremiah),
    ("Prayer of Azariah", Book::PrayerOfAzariah),
    ("PrAzariah", Book::PrayerOfAzariah),
    ("Susanna", Book::Susanna),
    ("First Maccabees", Book::FirstMaccabees),
    ("Second Maccabees", Book::SecondMaccabees),
    ("1 Maccabees", Book::FirstMaccabees),
    ("2 Maccabees", Book::SecondMaccabees),
    ("1 Macc", Book::FirstMaccabees),
    ("2 Macc", Book::SecondMaccabees),
    ("First Esdras", Book::FirstEsdras),
    ("Second Esdras", Book::SecondEsdras),
    ("Fourth Esdras", Book::FourthEsdras),
    ("1 Esdras", Book::FirstEsdras),
    ("2 Esdras", Book::SecondEsdras),
    ("4 Esdras", Book::FourthEsdras),
    ("Psalm 151", Book::Psalm151),
    ("Third Maccabees", Book::ThirdMaccabees),
    ("Fourth Maccabees", Book::FourthMaccabees),
    ("3 Maccabees", Book::ThirdMaccabees),
    ("4 Maccabees", Book::FourthMaccabees),
    ("3 Macc", Book::ThirdMaccabees),
    ("4 Macc", Book::FourthMaccabees),
    ("Bel", Book::Bel),
    ("Bel and the Dragon", Book::Bel),
    ("Judith", Book::Judith),
];
