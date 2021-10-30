// Scratch code for converting from Venite 2 JSON files 

function rType (reading_type) {
  switch(reading_type) {
    case "first_reading": 
            return "ReadingType::FirstReading";
    case "second_reading": 
            return "ReadingType::SecondReading";
    case "gospel": 
            return "ReadingType::Gospel";
    case "holy_day_morning_1": 
            return "ReadingType::Morning1";
    case "holy_day_morning_2": 
            return "ReadingType::Morning2";
    case "holy_day_evening_1": 
            return "ReadingType::Evening1";
    case "holy_day_evening_2": 
            return "ReadingType::Evening2";
    case "morning_psalms": 
            return "ReadingType::MorningPsalms";
    case "evening_psalms": 
            return "ReadingType::EveningPsalms";
  }
}

function makeId(day) {
  const split = day.split("-");
    if(["sunday","monday","tuesday","wednesday","thursday","friday","saturday"].includes(split[0])) {
    const week = split.slice(1).map(piece => `${piece[0].toUpperCase()}${piece.slice(1)}`).join("");
        const weekday = `${split[0][0].toUpperCase()}${split[0].slice(1)}`.slice(0, 3);
    return `LiturgicalDayId::WeekAndDay(LiturgicalWeek::${week}, Weekday::${weekday})`
  }
  else {
    const feast = split.map(piece => `${piece[0].toUpperCase()}${piece.slice(1)}`).join("");
    return `LiturgicalDayId::Feast(${feast})`;
  }
}

function makeYear(when) {
  return when ? when === "1" ? "Year::DailyOffice(DailyOfficeYear::One)" : "Year::DailyOffice(DailyOfficeYear::Two)" : "Year::Any";
}

l.map(entry => `(${makeId(entry.day)}, ${makeYear(entry.when)}, ${rType(entry.type)}, ${entry.citation})`)