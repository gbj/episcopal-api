function setDefaultValues() {
  // Sets date to today
  const now = new Date();
  document.getElementById("date").value = now.toLocaleDateString('en-CA');

  // Sets default liturgy based on time
  document.getElementById("liturgy").value = liturgyOfTheMoment(now, DEFAULT_RANGES);
}

const DEFAULT_RANGES = { morning: {
  start: { hour: 3, minute: 0 },
  end: { hour: 11, minute: 0 },
},
noon: {
  start: { hour: 11, minute: 0 },
  end: { hour: 14, minute: 0 },
},
evening: {
  start: { hour: 14, minute: 0 },
  end: { hour: 20, minute: 0 },
},
compline: {
  start: { hour: 20, minute: 0 },
  end: { hour: 3, minute: 0 },
}
}

function liturgyOfTheMoment(now, range) {
  const hour = now.getHours(),
    minute = now.getMinutes();
  if (
    (hour > range.morning.start.hour ||
      (hour === range.morning.start.hour &&
        minute > range.morning.start.minute)) &&
    (hour < range.morning.end.hour ||
      (hour === range.morning.end.hour && minute < range.morning.end.minute) ||
      range.morning.end.hour < range.morning.start.hour)
  ) {
    return "morning-prayer";
  } else if (
    (hour > range.noon.start.hour ||
      (hour === range.noon.start.hour && minute > range.noon.start.minute)) &&
    (hour < range.noon.end.hour ||
      (hour === range.noon.end.hour && minute < range.noon.end.minute) ||
      range.noon.end.hour < range.noon.start.hour)
  ) {
    return "noonday-prayer";
  } else if (
    (hour > range.evening.start.hour ||
      (hour === range.evening.start.hour &&
        minute > range.evening.start.minute)) &&
    (hour < range.evening.end.hour ||
      (hour === range.evening.end.hour && minute < range.evening.end.minute) ||
      range.evening.end.hour < range.evening.start.hour)
  ) {
    return "evening-prayer";
  } else if (
    (hour > range.compline.start.hour ||
      (hour === range.compline.start.hour &&
        minute > range.compline.start.minute)) &&
    (hour < range.compline.end.hour ||
      (hour === range.compline.end.hour &&
        minute < range.compline.end.minute) ||
      range.compline.end.hour < range.compline.start.hour)
  ) {
    return "compline";
  } else {
    return "morning-prayer";
  }
}

setDefaultValues();