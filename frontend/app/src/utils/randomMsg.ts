const words = [
    "Sustainable",
    "taiyaki",
    "semiotics",
    "mumblecore",
    "helvetica",
    "celiac",
    "selfies",
    "franzen",
    "etsy",
    "truffaut",
    "umami",
    "ugh",
    "swag",
    "fixie",
    "lumbersexual.",
    "Hoodie",
    "food",
    "truck",
    "sriracha",
    "squid",
    "selfies",
    "normcore",
    "cornhole",
    "marfa",
    "fingerstache",
    "twee",
    "flannel",
    "big",
    "mood.",
    "Try-hard",
    "bicycle",
    "rights",
    "tonx,",
    "celiac",
    "af",
    "3",
    "wolf",
    "moon",
    "craft",
    "beer",
    "etsy",
    "PBR&B",
    "man",
    "bun",
    "blue",
    "bottle",
    "fit",
    "Brooklyn",
    "mixtape.",
    "Activated",
    "charcoal",
    "taxidermy",
    "photo",
    "booth",
    "3",
    "wolf",
    "moon",
    "truffaut",
    "lumbersexual.",
];

export function randomSentence(min = 1, max = 50): string {
    const count = Math.floor(Math.random() * (max - min + 1)) + min;
    const shuffled = words.sort(() => 0.5 - Math.random());
    return shuffled.slice(0, count).join(" ");
}
