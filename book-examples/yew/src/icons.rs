use lucide_yew::{Component, *};
use yew::prelude::*;
#[function_component]
pub fn Icons() -> Html {
    html! {
        <div class="w-full max-w-80 py-4">
            <IconsA />
            <IconsB />
            <IconsC />
            <IconsD />
            <IconsE />
            <IconsF />
            <IconsG />
            <IconsH />
            <IconsI />
            <IconsJ />
            <IconsK />
            <IconsL />
            <IconsM />
            <IconsN />
            <IconsO />
            <IconsP />
            <IconsQ />
            <IconsR />
            <IconsS />
            <IconsT />
            <IconsU />
            <IconsV />
            <IconsW />
            <IconsX />
            <IconsY />
            <IconsZ />
        </div>
    }
}
#[function_component]
pub fn IconsA() -> Html {
    let icons = [
        (html! { <AArrowDown /> }, "A Arrow Down"),
        (html! { <AArrowUp /> }, "A Arrow Up"),
        (html! { <ALargeSmall /> }, "A Large Small"),
        (html! { <Accessibility /> }, "Accessibility"),
        (html! { <Activity /> }, "Activity"),
        (html! { <AirVent /> }, "Air Vent"),
        (html! { <Airplay /> }, "Airplay"),
        (html! { <AlarmClockCheck /> }, "Alarm Clock Check"),
        (html! { <AlarmClockMinus /> }, "Alarm Clock Minus"),
        (html! { <AlarmClockOff /> }, "Alarm Clock Off"),
        (html! { <AlarmClockPlus /> }, "Alarm Clock Plus"),
        (html! { <AlarmClock /> }, "Alarm Clock"),
        (html! { <AlarmSmoke /> }, "Alarm Smoke"),
        (html! { <Album /> }, "Album"),
        (
            html! { <AlignCenterHorizontal /> },
            "Align Center Horizontal",
        ),
        (html! { <AlignCenterVertical /> }, "Align Center Vertical"),
        (html! { <AlignCenter /> }, "Align Center"),
        (html! { <AlignEndHorizontal /> }, "Align End Horizontal"),
        (html! { <AlignEndVertical /> }, "Align End Vertical"),
        (
            html! { <AlignHorizontalDistributeCenter /> },
            "Align Horizontal Distribute Center",
        ),
        (
            html! { <AlignHorizontalDistributeEnd /> },
            "Align Horizontal Distribute End",
        ),
        (
            html! { <AlignHorizontalDistributeStart /> },
            "Align Horizontal Distribute Start",
        ),
        (
            html! { <AlignHorizontalJustifyCenter /> },
            "Align Horizontal Justify Center",
        ),
        (
            html! { <AlignHorizontalJustifyEnd /> },
            "Align Horizontal Justify End",
        ),
        (
            html! { <AlignHorizontalJustifyStart /> },
            "Align Horizontal Justify Start",
        ),
        (
            html! { <AlignHorizontalSpaceAround /> },
            "Align Horizontal Space Around",
        ),
        (
            html! { <AlignHorizontalSpaceBetween /> },
            "Align Horizontal Space Between",
        ),
        (html! { <AlignJustify /> }, "Align Justify"),
        (html! { <AlignLeft /> }, "Align Left"),
        (html! { <AlignRight /> }, "Align Right"),
        (html! { <AlignStartHorizontal /> }, "Align Start Horizontal"),
        (html! { <AlignStartVertical /> }, "Align Start Vertical"),
        (
            html! { <AlignVerticalDistributeCenter /> },
            "Align Vertical Distribute Center",
        ),
        (
            html! { <AlignVerticalDistributeEnd /> },
            "Align Vertical Distribute End",
        ),
        (
            html! { <AlignVerticalDistributeStart /> },
            "Align Vertical Distribute Start",
        ),
        (
            html! { <AlignVerticalJustifyCenter /> },
            "Align Vertical Justify Center",
        ),
        (
            html! { <AlignVerticalJustifyEnd /> },
            "Align Vertical Justify End",
        ),
        (
            html! { <AlignVerticalJustifyStart /> },
            "Align Vertical Justify Start",
        ),
        (
            html! { <AlignVerticalSpaceAround /> },
            "Align Vertical Space Around",
        ),
        (
            html! { <AlignVerticalSpaceBetween /> },
            "Align Vertical Space Between",
        ),
        (html! { <Ambulance /> }, "Ambulance"),
        (html! { <Ampersand /> }, "Ampersand"),
        (html! { <Ampersands /> }, "Ampersands"),
        (html! { <Amphora /> }, "Amphora"),
        (html! { <Anchor /> }, "Anchor"),
        (html! { <Angry /> }, "Angry"),
        (html! { <Annoyed /> }, "Annoyed"),
        (html! { <Antenna /> }, "Antenna"),
        (html! { <Anvil /> }, "Anvil"),
        (html! { <Aperture /> }, "Aperture"),
        (html! { <AppWindowMac /> }, "App Window Mac"),
        (html! { <AppWindow /> }, "App Window"),
        (html! { <Apple /> }, "Apple"),
        (html! { <ArchiveRestore /> }, "Archive Restore"),
        (html! { <ArchiveX /> }, "Archive X"),
        (html! { <Archive /> }, "Archive"),
        (html! { <Armchair /> }, "Armchair"),
        (html! { <ArrowBigDownDash /> }, "Arrow Big Down Dash"),
        (html! { <ArrowBigDown /> }, "Arrow Big Down"),
        (html! { <ArrowBigLeftDash /> }, "Arrow Big Left Dash"),
        (html! { <ArrowBigLeft /> }, "Arrow Big Left"),
        (html! { <ArrowBigRightDash /> }, "Arrow Big Right Dash"),
        (html! { <ArrowBigRight /> }, "Arrow Big Right"),
        (html! { <ArrowBigUpDash /> }, "Arrow Big Up Dash"),
        (html! { <ArrowBigUp /> }, "Arrow Big Up"),
        (html! { <ArrowDown01 /> }, "Arrow Down 01"),
        (html! { <ArrowDown10 /> }, "Arrow Down 10"),
        (html! { <ArrowDownAZ /> }, "Arrow Down Az"),
        (html! { <ArrowDownFromLine /> }, "Arrow Down From Line"),
        (html! { <ArrowDownLeft /> }, "Arrow Down Left"),
        (html! { <ArrowDownNarrowWide /> }, "Arrow Down Narrow Wide"),
        (html! { <ArrowDownRight /> }, "Arrow Down Right"),
        (html! { <ArrowDownToDot /> }, "Arrow Down To Dot"),
        (html! { <ArrowDownToLine /> }, "Arrow Down To Line"),
        (html! { <ArrowDownUp /> }, "Arrow Down Up"),
        (html! { <ArrowDownWideNarrow /> }, "Arrow Down Wide Narrow"),
        (html! { <ArrowDownZA /> }, "Arrow Down Za"),
        (html! { <ArrowDown /> }, "Arrow Down"),
        (html! { <ArrowLeftFromLine /> }, "Arrow Left From Line"),
        (html! { <ArrowLeftRight /> }, "Arrow Left Right"),
        (html! { <ArrowLeftToLine /> }, "Arrow Left To Line"),
        (html! { <ArrowLeft /> }, "Arrow Left"),
        (html! { <ArrowRightFromLine /> }, "Arrow Right From Line"),
        (html! { <ArrowRightLeft /> }, "Arrow Right Left"),
        (html! { <ArrowRightToLine /> }, "Arrow Right To Line"),
        (html! { <ArrowRight /> }, "Arrow Right"),
        (html! { <ArrowUp01 /> }, "Arrow Up 01"),
        (html! { <ArrowUp10 /> }, "Arrow Up 10"),
        (html! { <ArrowUpAZ /> }, "Arrow Up Az"),
        (html! { <ArrowUpDown /> }, "Arrow Up Down"),
        (html! { <ArrowUpFromDot /> }, "Arrow Up From Dot"),
        (html! { <ArrowUpFromLine /> }, "Arrow Up From Line"),
        (html! { <ArrowUpLeft /> }, "Arrow Up Left"),
        (html! { <ArrowUpNarrowWide /> }, "Arrow Up Narrow Wide"),
        (html! { <ArrowUpRight /> }, "Arrow Up Right"),
        (html! { <ArrowUpToLine /> }, "Arrow Up To Line"),
        (html! { <ArrowUpWideNarrow /> }, "Arrow Up Wide Narrow"),
        (html! { <ArrowUpZA /> }, "Arrow Up Za"),
        (html! { <ArrowUp /> }, "Arrow Up"),
        (html! { <ArrowsUpFromLine /> }, "Arrows Up From Line"),
        (html! { <Asterisk /> }, "Asterisk"),
        (html! { <AtSign /> }, "At Sign"),
        (html! { <Atom /> }, "Atom"),
        (html! { <AudioLines /> }, "Audio Lines"),
        (html! { <AudioWaveform /> }, "Audio Waveform"),
        (html! { <Award /> }, "Award"),
        (html! { <Axe /> }, "Axe"),
        (html! { <Axis3D /> }, "Axis 3 D"),
    ];
    icons
        .into_iter()
        .map(|(icon, name)| {
            html! {
                <div class="flex flex-wrap items-center gap-4 text-sm">
                    { icon }
                    <span>{ name }</span>
                </div>
            }
        })
        .collect::<Html>()
}
#[function_component]
pub fn IconsB() -> Html {
    let icons = [
        (html! { <Baby /> }, "Baby"),
        (html! { <Backpack /> }, "Backpack"),
        (html! { <BadgeAlert /> }, "Badge Alert"),
        (html! { <BadgeCent /> }, "Badge Cent"),
        (html! { <BadgeCheck /> }, "Badge Check"),
        (html! { <BadgeDollarSign /> }, "Badge Dollar Sign"),
        (html! { <BadgeEuro /> }, "Badge Euro"),
        (html! { <BadgeHelp /> }, "Badge Help"),
        (html! { <BadgeIndianRupee /> }, "Badge Indian Rupee"),
        (html! { <BadgeInfo /> }, "Badge Info"),
        (html! { <BadgeJapaneseYen /> }, "Badge Japanese Yen"),
        (html! { <BadgeMinus /> }, "Badge Minus"),
        (html! { <BadgePercent /> }, "Badge Percent"),
        (html! { <BadgePlus /> }, "Badge Plus"),
        (html! { <BadgePoundSterling /> }, "Badge Pound Sterling"),
        (html! { <BadgeRussianRuble /> }, "Badge Russian Ruble"),
        (html! { <BadgeSwissFranc /> }, "Badge Swiss Franc"),
        (html! { <BadgeX /> }, "Badge X"),
        (html! { <Badge /> }, "Badge"),
        (html! { <BaggageClaim /> }, "Baggage Claim"),
        (html! { <Ban /> }, "Ban"),
        (html! { <Banana /> }, "Banana"),
        (html! { <Bandage /> }, "Bandage"),
        (html! { <Banknote /> }, "Banknote"),
        (html! { <Barcode /> }, "Barcode"),
        (html! { <Baseline /> }, "Baseline"),
        (html! { <Bath /> }, "Bath"),
        (html! { <BatteryCharging /> }, "Battery Charging"),
        (html! { <BatteryFull /> }, "Battery Full"),
        (html! { <BatteryLow /> }, "Battery Low"),
        (html! { <BatteryMedium /> }, "Battery Medium"),
        (html! { <BatteryWarning /> }, "Battery Warning"),
        (html! { <Battery /> }, "Battery"),
        (html! { <Beaker /> }, "Beaker"),
        (html! { <BeanOff /> }, "Bean Off"),
        (html! { <Bean /> }, "Bean"),
        (html! { <BedDouble /> }, "Bed Double"),
        (html! { <BedSingle /> }, "Bed Single"),
        (html! { <Bed /> }, "Bed"),
        (html! { <Beef /> }, "Beef"),
        (html! { <BeerOff /> }, "Beer Off"),
        (html! { <Beer /> }, "Beer"),
        (html! { <BellDot /> }, "Bell Dot"),
        (html! { <BellElectric /> }, "Bell Electric"),
        (html! { <BellMinus /> }, "Bell Minus"),
        (html! { <BellOff /> }, "Bell Off"),
        (html! { <BellPlus /> }, "Bell Plus"),
        (html! { <BellRing /> }, "Bell Ring"),
        (html! { <Bell /> }, "Bell"),
        (html! { <BetweenHorizontalEnd /> }, "Between Horizontal End"),
        (
            html! { <BetweenHorizontalStart /> },
            "Between Horizontal Start",
        ),
        (html! { <BetweenVerticalEnd /> }, "Between Vertical End"),
        (html! { <BetweenVerticalStart /> }, "Between Vertical Start"),
        (html! { <BicepsFlexed /> }, "Biceps Flexed"),
        (html! { <Bike /> }, "Bike"),
        (html! { <Binary /> }, "Binary"),
        (html! { <Binoculars /> }, "Binoculars"),
        (html! { <Biohazard /> }, "Biohazard"),
        (html! { <Bird /> }, "Bird"),
        (html! { <Bitcoin /> }, "Bitcoin"),
        (html! { <Blend /> }, "Blend"),
        (html! { <Blinds /> }, "Blinds"),
        (html! { <Blocks /> }, "Blocks"),
        (html! { <BluetoothConnected /> }, "Bluetooth Connected"),
        (html! { <BluetoothOff /> }, "Bluetooth Off"),
        (html! { <BluetoothSearching /> }, "Bluetooth Searching"),
        (html! { <Bluetooth /> }, "Bluetooth"),
        (html! { <Bold /> }, "Bold"),
        (html! { <Bolt /> }, "Bolt"),
        (html! { <Bomb /> }, "Bomb"),
        (html! { <Bone /> }, "Bone"),
        (html! { <BookA /> }, "Book A"),
        (html! { <BookAudio /> }, "Book Audio"),
        (html! { <BookCheck /> }, "Book Check"),
        (html! { <BookCopy /> }, "Book Copy"),
        (html! { <BookDashed /> }, "Book Dashed"),
        (html! { <BookDown /> }, "Book Down"),
        (html! { <BookHeadphones /> }, "Book Headphones"),
        (html! { <BookHeart /> }, "Book Heart"),
        (html! { <BookImage /> }, "Book Image"),
        (html! { <BookKey /> }, "Book Key"),
        (html! { <BookLock /> }, "Book Lock"),
        (html! { <BookMarked /> }, "Book Marked"),
        (html! { <BookMinus /> }, "Book Minus"),
        (html! { <BookOpenCheck /> }, "Book Open Check"),
        (html! { <BookOpenText /> }, "Book Open Text"),
        (html! { <BookOpen /> }, "Book Open"),
        (html! { <BookPlus /> }, "Book Plus"),
        (html! { <BookText /> }, "Book Text"),
        (html! { <BookType /> }, "Book Type"),
        (html! { <BookUp2 /> }, "Book Up 2"),
        (html! { <BookUp /> }, "Book Up"),
        (html! { <BookUser /> }, "Book User"),
        (html! { <BookX /> }, "Book X"),
        (html! { <Book /> }, "Book"),
        (html! { <BookmarkCheck /> }, "Bookmark Check"),
        (html! { <BookmarkMinus /> }, "Bookmark Minus"),
        (html! { <BookmarkPlus /> }, "Bookmark Plus"),
        (html! { <BookmarkX /> }, "Bookmark X"),
        (html! { <Bookmark /> }, "Bookmark"),
        (html! { <BoomBox /> }, "Boom Box"),
        (html! { <BotMessageSquare /> }, "Bot Message Square"),
        (html! { <BotOff /> }, "Bot Off"),
        (html! { <Bot /> }, "Bot"),
        (html! { <Box /> }, "Box"),
        (html! { <Boxes /> }, "Boxes"),
        (html! { <Braces /> }, "Braces"),
        (html! { <Brackets /> }, "Brackets"),
        (html! { <BrainCircuit /> }, "Brain Circuit"),
        (html! { <BrainCog /> }, "Brain Cog"),
        (html! { <Brain /> }, "Brain"),
        (html! { <BrickWall /> }, "Brick Wall"),
        (html! { <BriefcaseBusiness /> }, "Briefcase Business"),
        (
            html! { <BriefcaseConveyorBelt /> },
            "Briefcase Conveyor Belt",
        ),
        (html! { <BriefcaseMedical /> }, "Briefcase Medical"),
        (html! { <Briefcase /> }, "Briefcase"),
        (html! { <BringToFront /> }, "Bring To Front"),
        (html! { <Brush /> }, "Brush"),
        (html! { <BugOff /> }, "Bug Off"),
        (html! { <BugPlay /> }, "Bug Play"),
        (html! { <Bug /> }, "Bug"),
        (html! { <Building2 /> }, "Building 2"),
        (html! { <Building /> }, "Building"),
        (html! { <BusFront /> }, "Bus Front"),
        (html! { <Bus /> }, "Bus"),
    ];
    icons
        .into_iter()
        .map(|(icon, name)| {
            html! {
                <div class="flex flex-wrap items-center gap-4 text-sm">
                    { icon }
                    <span>{ name }</span>
                </div>
            }
        })
        .collect::<Html>()
}
#[function_component]
pub fn IconsC() -> Html {
    let icons = [
        (html! { <CableCar /> }, "Cable Car"),
        (html! { <Cable /> }, "Cable"),
        (html! { <CakeSlice /> }, "Cake Slice"),
        (html! { <Cake /> }, "Cake"),
        (html! { <Calculator /> }, "Calculator"),
        (html! { <Calendar1 /> }, "Calendar 1"),
        (html! { <CalendarArrowDown /> }, "Calendar Arrow Down"),
        (html! { <CalendarArrowUp /> }, "Calendar Arrow Up"),
        (html! { <CalendarCheck2 /> }, "Calendar Check 2"),
        (html! { <CalendarCheck /> }, "Calendar Check"),
        (html! { <CalendarClock /> }, "Calendar Clock"),
        (html! { <CalendarCog /> }, "Calendar Cog"),
        (html! { <CalendarDays /> }, "Calendar Days"),
        (html! { <CalendarFold /> }, "Calendar Fold"),
        (html! { <CalendarHeart /> }, "Calendar Heart"),
        (html! { <CalendarMinus2 /> }, "Calendar Minus 2"),
        (html! { <CalendarMinus /> }, "Calendar Minus"),
        (html! { <CalendarOff /> }, "Calendar Off"),
        (html! { <CalendarPlus2 /> }, "Calendar Plus 2"),
        (html! { <CalendarPlus /> }, "Calendar Plus"),
        (html! { <CalendarRange /> }, "Calendar Range"),
        (html! { <CalendarSearch /> }, "Calendar Search"),
        (html! { <CalendarX2 /> }, "Calendar X 2"),
        (html! { <CalendarX /> }, "Calendar X"),
        (html! { <Calendar /> }, "Calendar"),
        (html! { <CameraOff /> }, "Camera Off"),
        (html! { <Camera /> }, "Camera"),
        (html! { <CandyCane /> }, "Candy Cane"),
        (html! { <CandyOff /> }, "Candy Off"),
        (html! { <Candy /> }, "Candy"),
        (html! { <Cannabis /> }, "Cannabis"),
        (html! { <CaptionsOff /> }, "Captions Off"),
        (html! { <Captions /> }, "Captions"),
        (html! { <CarFront /> }, "Car Front"),
        (html! { <CarTaxiFront /> }, "Car Taxi Front"),
        (html! { <Car /> }, "Car"),
        (html! { <Caravan /> }, "Caravan"),
        (html! { <Carrot /> }, "Carrot"),
        (html! { <CaseLower /> }, "Case Lower"),
        (html! { <CaseSensitive /> }, "Case Sensitive"),
        (html! { <CaseUpper /> }, "Case Upper"),
        (html! { <CassetteTape /> }, "Cassette Tape"),
        (html! { <Cast /> }, "Cast"),
        (html! { <Castle /> }, "Castle"),
        (html! { <Cat /> }, "Cat"),
        (html! { <Cctv /> }, "Cctv"),
        (html! { <ChartArea /> }, "Chart Area"),
        (html! { <ChartBarBig /> }, "Chart Bar Big"),
        (html! { <ChartBarDecreasing /> }, "Chart Bar Decreasing"),
        (html! { <ChartBarIncreasing /> }, "Chart Bar Increasing"),
        (html! { <ChartBarStacked /> }, "Chart Bar Stacked"),
        (html! { <ChartBar /> }, "Chart Bar"),
        (html! { <ChartCandlestick /> }, "Chart Candlestick"),
        (html! { <ChartColumnBig /> }, "Chart Column Big"),
        (
            html! { <ChartColumnDecreasing /> },
            "Chart Column Decreasing",
        ),
        (
            html! { <ChartColumnIncreasing /> },
            "Chart Column Increasing",
        ),
        (html! { <ChartColumnStacked /> }, "Chart Column Stacked"),
        (html! { <ChartColumn /> }, "Chart Column"),
        (html! { <ChartGantt /> }, "Chart Gantt"),
        (html! { <ChartLine /> }, "Chart Line"),
        (html! { <ChartNetwork /> }, "Chart Network"),
        (
            html! { <ChartNoAxesColumnDecreasing /> },
            "Chart No Axes Column Decreasing",
        ),
        (
            html! { <ChartNoAxesColumnIncreasing /> },
            "Chart No Axes Column Increasing",
        ),
        (html! { <ChartNoAxesColumn /> }, "Chart No Axes Column"),
        (html! { <ChartNoAxesCombined /> }, "Chart No Axes Combined"),
        (html! { <ChartNoAxesGantt /> }, "Chart No Axes Gantt"),
        (html! { <ChartPie /> }, "Chart Pie"),
        (html! { <ChartScatter /> }, "Chart Scatter"),
        (html! { <ChartSpline /> }, "Chart Spline"),
        (html! { <CheckCheck /> }, "Check Check"),
        (html! { <Check /> }, "Check"),
        (html! { <ChefHat /> }, "Chef Hat"),
        (html! { <Cherry /> }, "Cherry"),
        (html! { <ChevronDown /> }, "Chevron Down"),
        (html! { <ChevronFirst /> }, "Chevron First"),
        (html! { <ChevronLast /> }, "Chevron Last"),
        (html! { <ChevronLeft /> }, "Chevron Left"),
        (html! { <ChevronRight /> }, "Chevron Right"),
        (html! { <ChevronUp /> }, "Chevron Up"),
        (html! { <ChevronsDownUp /> }, "Chevrons Down Up"),
        (html! { <ChevronsDown /> }, "Chevrons Down"),
        (
            html! { <ChevronsLeftRightEllipsis /> },
            "Chevrons Left Right Ellipsis",
        ),
        (html! { <ChevronsLeftRight /> }, "Chevrons Left Right"),
        (html! { <ChevronsLeft /> }, "Chevrons Left"),
        (html! { <ChevronsRightLeft /> }, "Chevrons Right Left"),
        (html! { <ChevronsRight /> }, "Chevrons Right"),
        (html! { <ChevronsUpDown /> }, "Chevrons Up Down"),
        (html! { <ChevronsUp /> }, "Chevrons Up"),
        (html! { <Chrome /> }, "Chrome"),
        (html! { <Church /> }, "Church"),
        (html! { <CigaretteOff /> }, "Cigarette Off"),
        (html! { <Cigarette /> }, "Cigarette"),
        (html! { <CircleAlert /> }, "Circle Alert"),
        (html! { <CircleArrowDown /> }, "Circle Arrow Down"),
        (html! { <CircleArrowLeft /> }, "Circle Arrow Left"),
        (
            html! { <CircleArrowOutDownLeft /> },
            "Circle Arrow Out Down Left",
        ),
        (
            html! { <CircleArrowOutDownRight /> },
            "Circle Arrow Out Down Right",
        ),
        (
            html! { <CircleArrowOutUpLeft /> },
            "Circle Arrow Out Up Left",
        ),
        (
            html! { <CircleArrowOutUpRight /> },
            "Circle Arrow Out Up Right",
        ),
        (html! { <CircleArrowRight /> }, "Circle Arrow Right"),
        (html! { <CircleArrowUp /> }, "Circle Arrow Up"),
        (html! { <CircleCheckBig /> }, "Circle Check Big"),
        (html! { <CircleCheck /> }, "Circle Check"),
        (html! { <CircleChevronDown /> }, "Circle Chevron Down"),
        (html! { <CircleChevronLeft /> }, "Circle Chevron Left"),
        (html! { <CircleChevronRight /> }, "Circle Chevron Right"),
        (html! { <CircleChevronUp /> }, "Circle Chevron Up"),
        (html! { <CircleDashed /> }, "Circle Dashed"),
        (html! { <CircleDivide /> }, "Circle Divide"),
        (html! { <CircleDollarSign /> }, "Circle Dollar Sign"),
        (html! { <CircleDotDashed /> }, "Circle Dot Dashed"),
        (html! { <CircleDot /> }, "Circle Dot"),
        (html! { <CircleEllipsis /> }, "Circle Ellipsis"),
        (html! { <CircleEqual /> }, "Circle Equal"),
        (html! { <CircleFadingArrowUp /> }, "Circle Fading Arrow Up"),
        (html! { <CircleFadingPlus /> }, "Circle Fading Plus"),
        (html! { <CircleGauge /> }, "Circle Gauge"),
        (html! { <CircleHelp /> }, "Circle Help"),
        (html! { <CircleMinus /> }, "Circle Minus"),
        (html! { <CircleOff /> }, "Circle Off"),
        (html! { <CircleParkingOff /> }, "Circle Parking Off"),
        (html! { <CircleParking /> }, "Circle Parking"),
        (html! { <CirclePause /> }, "Circle Pause"),
        (html! { <CirclePercent /> }, "Circle Percent"),
        (html! { <CirclePlay /> }, "Circle Play"),
        (html! { <CirclePlus /> }, "Circle Plus"),
        (html! { <CirclePower /> }, "Circle Power"),
        (html! { <CircleSlash2 /> }, "Circle Slash 2"),
        (html! { <CircleSlash /> }, "Circle Slash"),
        (html! { <CircleStop /> }, "Circle Stop"),
        (html! { <CircleUserRound /> }, "Circle User Round"),
        (html! { <CircleUser /> }, "Circle User"),
        (html! { <CircleX /> }, "Circle X"),
        (html! { <Circle /> }, "Circle"),
        (html! { <CircuitBoard /> }, "Circuit Board"),
        (html! { <Citrus /> }, "Citrus"),
        (html! { <Clapperboard /> }, "Clapperboard"),
        (html! { <ClipboardCheck /> }, "Clipboard Check"),
        (html! { <ClipboardCopy /> }, "Clipboard Copy"),
        (html! { <ClipboardList /> }, "Clipboard List"),
        (html! { <ClipboardMinus /> }, "Clipboard Minus"),
        (html! { <ClipboardPaste /> }, "Clipboard Paste"),
        (html! { <ClipboardPenLine /> }, "Clipboard Pen Line"),
        (html! { <ClipboardPen /> }, "Clipboard Pen"),
        (html! { <ClipboardPlus /> }, "Clipboard Plus"),
        (html! { <ClipboardType /> }, "Clipboard Type"),
        (html! { <ClipboardX /> }, "Clipboard X"),
        (html! { <Clipboard /> }, "Clipboard"),
        (html! { <Clock1 /> }, "Clock 1"),
        (html! { <Clock10 /> }, "Clock 10"),
        (html! { <Clock11 /> }, "Clock 11"),
        (html! { <Clock12 /> }, "Clock 12"),
        (html! { <Clock2 /> }, "Clock 2"),
        (html! { <Clock3 /> }, "Clock 3"),
        (html! { <Clock4 /> }, "Clock 4"),
        (html! { <Clock5 /> }, "Clock 5"),
        (html! { <Clock6 /> }, "Clock 6"),
        (html! { <Clock7 /> }, "Clock 7"),
        (html! { <Clock8 /> }, "Clock 8"),
        (html! { <Clock9 /> }, "Clock 9"),
        (html! { <ClockAlert /> }, "Clock Alert"),
        (html! { <ClockArrowDown /> }, "Clock Arrow Down"),
        (html! { <ClockArrowUp /> }, "Clock Arrow Up"),
        (html! { <Clock /> }, "Clock"),
        (html! { <CloudAlert /> }, "Cloud Alert"),
        (html! { <CloudCog /> }, "Cloud Cog"),
        (html! { <CloudDownload /> }, "Cloud Download"),
        (html! { <CloudDrizzle /> }, "Cloud Drizzle"),
        (html! { <CloudFog /> }, "Cloud Fog"),
        (html! { <CloudHail /> }, "Cloud Hail"),
        (html! { <CloudLightning /> }, "Cloud Lightning"),
        (html! { <CloudMoonRain /> }, "Cloud Moon Rain"),
        (html! { <CloudMoon /> }, "Cloud Moon"),
        (html! { <CloudOff /> }, "Cloud Off"),
        (html! { <CloudRainWind /> }, "Cloud Rain Wind"),
        (html! { <CloudRain /> }, "Cloud Rain"),
        (html! { <CloudSnow /> }, "Cloud Snow"),
        (html! { <CloudSunRain /> }, "Cloud Sun Rain"),
        (html! { <CloudSun /> }, "Cloud Sun"),
        (html! { <CloudUpload /> }, "Cloud Upload"),
        (html! { <Cloud /> }, "Cloud"),
        (html! { <Cloudy /> }, "Cloudy"),
        (html! { <Clover /> }, "Clover"),
        (html! { <Club /> }, "Club"),
        (html! { <CodeXml /> }, "Code Xml"),
        (html! { <Code /> }, "Code"),
        (html! { <Codepen /> }, "Codepen"),
        (html! { <Codesandbox /> }, "Codesandbox"),
        (html! { <Coffee /> }, "Coffee"),
        (html! { <Cog /> }, "Cog"),
        (html! { <Coins /> }, "Coins"),
        (html! { <Columns2 /> }, "Columns 2"),
        (html! { <Columns3 /> }, "Columns 3"),
        (html! { <Columns4 /> }, "Columns 4"),
        (html! { <Combine /> }, "Combine"),
        (html! { <Command /> }, "Command"),
        (html! { <Compass /> }, "Compass"),
        (html! { <Component /> }, "Component"),
        (html! { <Computer /> }, "Computer"),
        (html! { <ConciergeBell /> }, "Concierge Bell"),
        (html! { <Cone /> }, "Cone"),
        (html! { <Construction /> }, "Construction"),
        (html! { <ContactRound /> }, "Contact Round"),
        (html! { <Contact /> }, "Contact"),
        (html! { <Container /> }, "Container"),
        (html! { <Contrast /> }, "Contrast"),
        (html! { <Cookie /> }, "Cookie"),
        (html! { <CookingPot /> }, "Cooking Pot"),
        (html! { <CopyCheck /> }, "Copy Check"),
        (html! { <CopyMinus /> }, "Copy Minus"),
        (html! { <CopyPlus /> }, "Copy Plus"),
        (html! { <CopySlash /> }, "Copy Slash"),
        (html! { <CopyX /> }, "Copy X"),
        (html! { <Copy /> }, "Copy"),
        (html! { <Copyleft /> }, "Copyleft"),
        (html! { <Copyright /> }, "Copyright"),
        (html! { <CornerDownLeft /> }, "Corner Down Left"),
        (html! { <CornerDownRight /> }, "Corner Down Right"),
        (html! { <CornerLeftDown /> }, "Corner Left Down"),
        (html! { <CornerLeftUp /> }, "Corner Left Up"),
        (html! { <CornerRightDown /> }, "Corner Right Down"),
        (html! { <CornerRightUp /> }, "Corner Right Up"),
        (html! { <CornerUpLeft /> }, "Corner Up Left"),
        (html! { <CornerUpRight /> }, "Corner Up Right"),
        (html! { <Cpu /> }, "Cpu"),
        (html! { <CreativeCommons /> }, "Creative Commons"),
        (html! { <CreditCard /> }, "Credit Card"),
        (html! { <Croissant /> }, "Croissant"),
        (html! { <Crop /> }, "Crop"),
        (html! { <Cross /> }, "Cross"),
        (html! { <Crosshair /> }, "Crosshair"),
        (html! { <Crown /> }, "Crown"),
        (html! { <Cuboid /> }, "Cuboid"),
        (html! { <CupSoda /> }, "Cup Soda"),
        (html! { <Currency /> }, "Currency"),
        (html! { <Cylinder /> }, "Cylinder"),
    ];
    icons
        .into_iter()
        .map(|(icon, name)| {
            html! {
                <div class="flex flex-wrap items-center gap-4 text-sm">
                    { icon }
                    <span>{ name }</span>
                </div>
            }
        })
        .collect::<Html>()
}
#[function_component]
pub fn IconsD() -> Html {
    let icons = [
        (html! { <Dam /> }, "Dam"),
        (html! { <DatabaseBackup /> }, "Database Backup"),
        (html! { <DatabaseZap /> }, "Database Zap"),
        (html! { <Database /> }, "Database"),
        (html! { <Delete /> }, "Delete"),
        (html! { <Dessert /> }, "Dessert"),
        (html! { <Diameter /> }, "Diameter"),
        (html! { <DiamondMinus /> }, "Diamond Minus"),
        (html! { <DiamondPercent /> }, "Diamond Percent"),
        (html! { <DiamondPlus /> }, "Diamond Plus"),
        (html! { <Diamond /> }, "Diamond"),
        (html! { <Dice1 /> }, "Dice 1"),
        (html! { <Dice2 /> }, "Dice 2"),
        (html! { <Dice3 /> }, "Dice 3"),
        (html! { <Dice4 /> }, "Dice 4"),
        (html! { <Dice5 /> }, "Dice 5"),
        (html! { <Dice6 /> }, "Dice 6"),
        (html! { <Dices /> }, "Dices"),
        (html! { <Diff /> }, "Diff"),
        (html! { <Disc2 /> }, "Disc 2"),
        (html! { <Disc3 /> }, "Disc 3"),
        (html! { <DiscAlbum /> }, "Disc Album"),
        (html! { <Disc /> }, "Disc"),
        (html! { <Divide /> }, "Divide"),
        (html! { <DnaOff /> }, "Dna Off"),
        (html! { <Dna /> }, "Dna"),
        (html! { <Dock /> }, "Dock"),
        (html! { <Dog /> }, "Dog"),
        (html! { <DollarSign /> }, "Dollar Sign"),
        (html! { <Donut /> }, "Donut"),
        (html! { <DoorClosed /> }, "Door Closed"),
        (html! { <DoorOpen /> }, "Door Open"),
        (html! { <Dot /> }, "Dot"),
        (html! { <Download /> }, "Download"),
        (html! { <DraftingCompass /> }, "Drafting Compass"),
        (html! { <Drama /> }, "Drama"),
        (html! { <Dribbble /> }, "Dribbble"),
        (html! { <Drill /> }, "Drill"),
        (html! { <Droplet /> }, "Droplet"),
        (html! { <Droplets /> }, "Droplets"),
        (html! { <Drum /> }, "Drum"),
        (html! { <Drumstick /> }, "Drumstick"),
        (html! { <Dumbbell /> }, "Dumbbell"),
    ];
    icons
        .into_iter()
        .map(|(icon, name)| {
            html! {
                <div class="flex flex-wrap items-center gap-4 text-sm">
                    { icon }
                    <span>{ name }</span>
                </div>
            }
        })
        .collect::<Html>()
}
#[function_component]
pub fn IconsE() -> Html {
    let icons = [
        (html! { <EarOff /> }, "Ear Off"),
        (html! { <Ear /> }, "Ear"),
        (html! { <EarthLock /> }, "Earth Lock"),
        (html! { <Earth /> }, "Earth"),
        (html! { <Eclipse /> }, "Eclipse"),
        (html! { <EggFried /> }, "Egg Fried"),
        (html! { <EggOff /> }, "Egg Off"),
        (html! { <Egg /> }, "Egg"),
        (html! { <EllipsisVertical /> }, "Ellipsis Vertical"),
        (html! { <Ellipsis /> }, "Ellipsis"),
        (html! { <EqualApproximately /> }, "Equal Approximately"),
        (html! { <EqualNot /> }, "Equal Not"),
        (html! { <Equal /> }, "Equal"),
        (html! { <Eraser /> }, "Eraser"),
        (html! { <EthernetPort /> }, "Ethernet Port"),
        (html! { <Euro /> }, "Euro"),
        (html! { <Expand /> }, "Expand"),
        (html! { <ExternalLink /> }, "External Link"),
        (html! { <EyeClosed /> }, "Eye Closed"),
        (html! { <EyeOff /> }, "Eye Off"),
        (html! { <Eye /> }, "Eye"),
    ];
    icons
        .into_iter()
        .map(|(icon, name)| {
            html! {
                <div class="flex flex-wrap items-center gap-4 text-sm">
                    { icon }
                    <span>{ name }</span>
                </div>
            }
        })
        .collect::<Html>()
}
#[function_component]
pub fn IconsF() -> Html {
    let icons = [
        (html! { <Facebook /> }, "Facebook"),
        (html! { <Factory /> }, "Factory"),
        (html! { <Fan /> }, "Fan"),
        (html! { <FastForward /> }, "Fast Forward"),
        (html! { <Feather /> }, "Feather"),
        (html! { <Fence /> }, "Fence"),
        (html! { <FerrisWheel /> }, "Ferris Wheel"),
        (html! { <Figma /> }, "Figma"),
        (html! { <FileArchive /> }, "File Archive"),
        (html! { <FileAudio2 /> }, "File Audio 2"),
        (html! { <FileAudio /> }, "File Audio"),
        (html! { <FileAxis3D /> }, "File Axis 3 D"),
        (html! { <FileBadge2 /> }, "File Badge 2"),
        (html! { <FileBadge /> }, "File Badge"),
        (html! { <FileBox /> }, "File Box"),
        (
            html! { <FileChartColumnIncreasing /> },
            "File Chart Column Increasing",
        ),
        (html! { <FileChartColumn /> }, "File Chart Column"),
        (html! { <FileChartLine /> }, "File Chart Line"),
        (html! { <FileChartPie /> }, "File Chart Pie"),
        (html! { <FileCheck2 /> }, "File Check 2"),
        (html! { <FileCheck /> }, "File Check"),
        (html! { <FileClock /> }, "File Clock"),
        (html! { <FileCode2 /> }, "File Code 2"),
        (html! { <FileCode /> }, "File Code"),
        (html! { <FileCog /> }, "File Cog"),
        (html! { <FileDiff /> }, "File Diff"),
        (html! { <FileDigit /> }, "File Digit"),
        (html! { <FileDown /> }, "File Down"),
        (html! { <FileHeart /> }, "File Heart"),
        (html! { <FileImage /> }, "File Image"),
        (html! { <FileInput /> }, "File Input"),
        (html! { <FileJson2 /> }, "File Json 2"),
        (html! { <FileJson /> }, "File Json"),
        (html! { <FileKey2 /> }, "File Key 2"),
        (html! { <FileKey /> }, "File Key"),
        (html! { <FileLock2 /> }, "File Lock 2"),
        (html! { <FileLock /> }, "File Lock"),
        (html! { <FileMinus2 /> }, "File Minus 2"),
        (html! { <FileMinus /> }, "File Minus"),
        (html! { <FileMusic /> }, "File Music"),
        (html! { <FileOutput /> }, "File Output"),
        (html! { <FilePenLine /> }, "File Pen Line"),
        (html! { <FilePen /> }, "File Pen"),
        (html! { <FilePlus2 /> }, "File Plus 2"),
        (html! { <FilePlus /> }, "File Plus"),
        (html! { <FileQuestion /> }, "File Question"),
        (html! { <FileScan /> }, "File Scan"),
        (html! { <FileSearch2 /> }, "File Search 2"),
        (html! { <FileSearch /> }, "File Search"),
        (html! { <FileSliders /> }, "File Sliders"),
        (html! { <FileSpreadsheet /> }, "File Spreadsheet"),
        (html! { <FileStack /> }, "File Stack"),
        (html! { <FileSymlink /> }, "File Symlink"),
        (html! { <FileTerminal /> }, "File Terminal"),
        (html! { <FileText /> }, "File Text"),
        (html! { <FileType2 /> }, "File Type 2"),
        (html! { <FileType /> }, "File Type"),
        (html! { <FileUp /> }, "File Up"),
        (html! { <FileUser /> }, "File User"),
        (html! { <FileVideo2 /> }, "File Video 2"),
        (html! { <FileVideo /> }, "File Video"),
        (html! { <FileVolume2 /> }, "File Volume 2"),
        (html! { <FileVolume /> }, "File Volume"),
        (html! { <FileWarning /> }, "File Warning"),
        (html! { <FileX2 /> }, "File X 2"),
        (html! { <FileX /> }, "File X"),
        (html! { <File /> }, "File"),
        (html! { <Files /> }, "Files"),
        (html! { <Film /> }, "Film"),
        (html! { <FilterX /> }, "Filter X"),
        (html! { <Filter /> }, "Filter"),
        (html! { <Fingerprint /> }, "Fingerprint"),
        (html! { <FireExtinguisher /> }, "Fire Extinguisher"),
        (html! { <FishOff /> }, "Fish Off"),
        (html! { <FishSymbol /> }, "Fish Symbol"),
        (html! { <Fish /> }, "Fish"),
        (html! { <FlagOff /> }, "Flag Off"),
        (html! { <FlagTriangleLeft /> }, "Flag Triangle Left"),
        (html! { <FlagTriangleRight /> }, "Flag Triangle Right"),
        (html! { <Flag /> }, "Flag"),
        (html! { <FlameKindling /> }, "Flame Kindling"),
        (html! { <Flame /> }, "Flame"),
        (html! { <FlashlightOff /> }, "Flashlight Off"),
        (html! { <Flashlight /> }, "Flashlight"),
        (html! { <FlaskConicalOff /> }, "Flask Conical Off"),
        (html! { <FlaskConical /> }, "Flask Conical"),
        (html! { <FlaskRound /> }, "Flask Round"),
        (html! { <FlipHorizontal2 /> }, "Flip Horizontal 2"),
        (html! { <FlipHorizontal /> }, "Flip Horizontal"),
        (html! { <FlipVertical2 /> }, "Flip Vertical 2"),
        (html! { <FlipVertical /> }, "Flip Vertical"),
        (html! { <Flower2 /> }, "Flower 2"),
        (html! { <Flower /> }, "Flower"),
        (html! { <Focus /> }, "Focus"),
        (html! { <FoldHorizontal /> }, "Fold Horizontal"),
        (html! { <FoldVertical /> }, "Fold Vertical"),
        (html! { <FolderArchive /> }, "Folder Archive"),
        (html! { <FolderCheck /> }, "Folder Check"),
        (html! { <FolderClock /> }, "Folder Clock"),
        (html! { <FolderClosed /> }, "Folder Closed"),
        (html! { <FolderCode /> }, "Folder Code"),
        (html! { <FolderCog /> }, "Folder Cog"),
        (html! { <FolderDot /> }, "Folder Dot"),
        (html! { <FolderDown /> }, "Folder Down"),
        (html! { <FolderGit2 /> }, "Folder Git 2"),
        (html! { <FolderGit /> }, "Folder Git"),
        (html! { <FolderHeart /> }, "Folder Heart"),
        (html! { <FolderInput /> }, "Folder Input"),
        (html! { <FolderKanban /> }, "Folder Kanban"),
        (html! { <FolderKey /> }, "Folder Key"),
        (html! { <FolderLock /> }, "Folder Lock"),
        (html! { <FolderMinus /> }, "Folder Minus"),
        (html! { <FolderOpenDot /> }, "Folder Open Dot"),
        (html! { <FolderOpen /> }, "Folder Open"),
        (html! { <FolderOutput /> }, "Folder Output"),
        (html! { <FolderPen /> }, "Folder Pen"),
        (html! { <FolderPlus /> }, "Folder Plus"),
        (html! { <FolderRoot /> }, "Folder Root"),
        (html! { <FolderSearch2 /> }, "Folder Search 2"),
        (html! { <FolderSearch /> }, "Folder Search"),
        (html! { <FolderSymlink /> }, "Folder Symlink"),
        (html! { <FolderSync /> }, "Folder Sync"),
        (html! { <FolderTree /> }, "Folder Tree"),
        (html! { <FolderUp /> }, "Folder Up"),
        (html! { <FolderX /> }, "Folder X"),
        (html! { <Folder /> }, "Folder"),
        (html! { <Folders /> }, "Folders"),
        (html! { <Footprints /> }, "Footprints"),
        (html! { <Forklift /> }, "Forklift"),
        (html! { <Forward /> }, "Forward"),
        (html! { <Frame /> }, "Frame"),
        (html! { <Framer /> }, "Framer"),
        (html! { <Frown /> }, "Frown"),
        (html! { <Fuel /> }, "Fuel"),
        (html! { <Fullscreen /> }, "Fullscreen"),
    ];
    icons
        .into_iter()
        .map(|(icon, name)| {
            html! {
                <div class="flex flex-wrap items-center gap-4 text-sm">
                    { icon }
                    <span>{ name }</span>
                </div>
            }
        })
        .collect::<Html>()
}
#[function_component]
pub fn IconsG() -> Html {
    let icons = [
        (html! { <GalleryHorizontalEnd /> }, "Gallery Horizontal End"),
        (html! { <GalleryHorizontal /> }, "Gallery Horizontal"),
        (html! { <GalleryThumbnails /> }, "Gallery Thumbnails"),
        (html! { <GalleryVerticalEnd /> }, "Gallery Vertical End"),
        (html! { <GalleryVertical /> }, "Gallery Vertical"),
        (html! { <Gamepad2 /> }, "Gamepad 2"),
        (html! { <Gamepad /> }, "Gamepad"),
        (html! { <Gauge /> }, "Gauge"),
        (html! { <Gavel /> }, "Gavel"),
        (html! { <Gem /> }, "Gem"),
        (html! { <Ghost /> }, "Ghost"),
        (html! { <Gift /> }, "Gift"),
        (html! { <GitBranchPlus /> }, "Git Branch Plus"),
        (html! { <GitBranch /> }, "Git Branch"),
        (html! { <GitCommitHorizontal /> }, "Git Commit Horizontal"),
        (html! { <GitCommitVertical /> }, "Git Commit Vertical"),
        (html! { <GitCompareArrows /> }, "Git Compare Arrows"),
        (html! { <GitCompare /> }, "Git Compare"),
        (html! { <GitFork /> }, "Git Fork"),
        (html! { <GitGraph /> }, "Git Graph"),
        (html! { <GitMerge /> }, "Git Merge"),
        (html! { <GitPullRequestArrow /> }, "Git Pull Request Arrow"),
        (
            html! { <GitPullRequestClosed /> },
            "Git Pull Request Closed",
        ),
        (
            html! { <GitPullRequestCreateArrow /> },
            "Git Pull Request Create Arrow",
        ),
        (
            html! { <GitPullRequestCreate /> },
            "Git Pull Request Create",
        ),
        (html! { <GitPullRequestDraft /> }, "Git Pull Request Draft"),
        (html! { <GitPullRequest /> }, "Git Pull Request"),
        (html! { <Github /> }, "Github"),
        (html! { <Gitlab /> }, "Gitlab"),
        (html! { <GlassWater /> }, "Glass Water"),
        (html! { <Glasses /> }, "Glasses"),
        (html! { <GlobeLock /> }, "Globe Lock"),
        (html! { <Globe /> }, "Globe"),
        (html! { <Goal /> }, "Goal"),
        (html! { <Grab /> }, "Grab"),
        (html! { <GraduationCap /> }, "Graduation Cap"),
        (html! { <Grape /> }, "Grape"),
        (html! { <Grid2X2Check /> }, "Grid 2 X 2 Check"),
        (html! { <Grid2X2Plus /> }, "Grid 2 X 2 Plus"),
        (html! { <Grid2X2X /> }, "Grid 2 X 2 X"),
        (html! { <Grid2X2 /> }, "Grid 2 X 2"),
        (html! { <Grid3X3 /> }, "Grid 3 X 3"),
        (html! { <GripHorizontal /> }, "Grip Horizontal"),
        (html! { <GripVertical /> }, "Grip Vertical"),
        (html! { <Grip /> }, "Grip"),
        (html! { <Group /> }, "Group"),
        (html! { <Guitar /> }, "Guitar"),
    ];
    icons
        .into_iter()
        .map(|(icon, name)| {
            html! {
                <div class="flex flex-wrap items-center gap-4 text-sm">
                    { icon }
                    <span>{ name }</span>
                </div>
            }
        })
        .collect::<Html>()
}
#[function_component]
pub fn IconsH() -> Html {
    let icons = [
        (html! { <Ham /> }, "Ham"),
        (html! { <Hammer /> }, "Hammer"),
        (html! { <HandCoins /> }, "Hand Coins"),
        (html! { <HandHeart /> }, "Hand Heart"),
        (html! { <HandHelping /> }, "Hand Helping"),
        (html! { <HandMetal /> }, "Hand Metal"),
        (html! { <HandPlatter /> }, "Hand Platter"),
        (html! { <Hand /> }, "Hand"),
        (html! { <Handshake /> }, "Handshake"),
        (html! { <HardDriveDownload /> }, "Hard Drive Download"),
        (html! { <HardDriveUpload /> }, "Hard Drive Upload"),
        (html! { <HardDrive /> }, "Hard Drive"),
        (html! { <HardHat /> }, "Hard Hat"),
        (html! { <Hash /> }, "Hash"),
        (html! { <Haze /> }, "Haze"),
        (html! { <HdmiPort /> }, "Hdmi Port"),
        (html! { <Heading1 /> }, "Heading 1"),
        (html! { <Heading2 /> }, "Heading 2"),
        (html! { <Heading3 /> }, "Heading 3"),
        (html! { <Heading4 /> }, "Heading 4"),
        (html! { <Heading5 /> }, "Heading 5"),
        (html! { <Heading6 /> }, "Heading 6"),
        (html! { <Heading /> }, "Heading"),
        (html! { <HeadphoneOff /> }, "Headphone Off"),
        (html! { <Headphones /> }, "Headphones"),
        (html! { <Headset /> }, "Headset"),
        (html! { <HeartCrack /> }, "Heart Crack"),
        (html! { <HeartHandshake /> }, "Heart Handshake"),
        (html! { <HeartOff /> }, "Heart Off"),
        (html! { <HeartPulse /> }, "Heart Pulse"),
        (html! { <Heart /> }, "Heart"),
        (html! { <Heater /> }, "Heater"),
        (html! { <Hexagon /> }, "Hexagon"),
        (html! { <Highlighter /> }, "Highlighter"),
        (html! { <History /> }, "History"),
        (html! { <HopOff /> }, "Hop Off"),
        (html! { <Hop /> }, "Hop"),
        (html! { <Hospital /> }, "Hospital"),
        (html! { <Hotel /> }, "Hotel"),
        (html! { <Hourglass /> }, "Hourglass"),
        (html! { <HousePlug /> }, "House Plug"),
        (html! { <HousePlus /> }, "House Plus"),
        (html! { <House /> }, "House"),
    ];
    icons
        .into_iter()
        .map(|(icon, name)| {
            html! {
                <div class="flex flex-wrap items-center gap-4 text-sm">
                    { icon }
                    <span>{ name }</span>
                </div>
            }
        })
        .collect::<Html>()
}
#[function_component]
pub fn IconsI() -> Html {
    let icons = [
        (html! { <IceCreamBowl /> }, "Ice Cream Bowl"),
        (html! { <IceCreamCone /> }, "Ice Cream Cone"),
        (html! { <IdCard /> }, "Id Card"),
        (html! { <ImageDown /> }, "Image Down"),
        (html! { <ImageMinus /> }, "Image Minus"),
        (html! { <ImageOff /> }, "Image Off"),
        (html! { <ImagePlay /> }, "Image Play"),
        (html! { <ImagePlus /> }, "Image Plus"),
        (html! { <ImageUp /> }, "Image Up"),
        (html! { <Image /> }, "Image"),
        (html! { <Images /> }, "Images"),
        (html! { <Import /> }, "Import"),
        (html! { <Inbox /> }, "Inbox"),
        (html! { <IndentDecrease /> }, "Indent Decrease"),
        (html! { <IndentIncrease /> }, "Indent Increase"),
        (html! { <IndianRupee /> }, "Indian Rupee"),
        (html! { <Infinity /> }, "Infinity"),
        (html! { <Info /> }, "Info"),
        (html! { <InspectionPanel /> }, "Inspection Panel"),
        (html! { <Instagram /> }, "Instagram"),
        (html! { <Italic /> }, "Italic"),
        (html! { <IterationCcw /> }, "Iteration Ccw"),
        (html! { <IterationCw /> }, "Iteration Cw"),
    ];
    icons
        .into_iter()
        .map(|(icon, name)| {
            html! {
                <div class="flex flex-wrap items-center gap-4 text-sm">
                    { icon }
                    <span>{ name }</span>
                </div>
            }
        })
        .collect::<Html>()
}
#[function_component]
pub fn IconsJ() -> Html {
    let icons = [
        (html! { <JapaneseYen /> }, "Japanese Yen"),
        (html! { <Joystick /> }, "Joystick"),
    ];
    icons
        .into_iter()
        .map(|(icon, name)| {
            html! {
                <div class="flex flex-wrap items-center gap-4 text-sm">
                    { icon }
                    <span>{ name }</span>
                </div>
            }
        })
        .collect::<Html>()
}
#[function_component]
pub fn IconsK() -> Html {
    let icons = [
        (html! { <Kanban /> }, "Kanban"),
        (html! { <KeyRound /> }, "Key Round"),
        (html! { <KeySquare /> }, "Key Square"),
        (html! { <Key /> }, "Key"),
        (html! { <KeyboardMusic /> }, "Keyboard Music"),
        (html! { <KeyboardOff /> }, "Keyboard Off"),
        (html! { <Keyboard /> }, "Keyboard"),
    ];
    icons
        .into_iter()
        .map(|(icon, name)| {
            html! {
                <div class="flex flex-wrap items-center gap-4 text-sm">
                    { icon }
                    <span>{ name }</span>
                </div>
            }
        })
        .collect::<Html>()
}
#[function_component]
pub fn IconsL() -> Html {
    let icons = [
        (html! { <LampCeiling /> }, "Lamp Ceiling"),
        (html! { <LampDesk /> }, "Lamp Desk"),
        (html! { <LampFloor /> }, "Lamp Floor"),
        (html! { <LampWallDown /> }, "Lamp Wall Down"),
        (html! { <LampWallUp /> }, "Lamp Wall Up"),
        (html! { <Lamp /> }, "Lamp"),
        (html! { <LandPlot /> }, "Land Plot"),
        (html! { <Landmark /> }, "Landmark"),
        (html! { <Languages /> }, "Languages"),
        (html! { <LaptopMinimalCheck /> }, "Laptop Minimal Check"),
        (html! { <LaptopMinimal /> }, "Laptop Minimal"),
        (html! { <Laptop /> }, "Laptop"),
        (html! { <LassoSelect /> }, "Lasso Select"),
        (html! { <Lasso /> }, "Lasso"),
        (html! { <Laugh /> }, "Laugh"),
        (html! { <Layers2 /> }, "Layers 2"),
        (html! { <Layers3 /> }, "Layers 3"),
        (html! { <Layers /> }, "Layers"),
        (html! { <LayoutDashboard /> }, "Layout Dashboard"),
        (html! { <LayoutGrid /> }, "Layout Grid"),
        (html! { <LayoutList /> }, "Layout List"),
        (html! { <LayoutPanelLeft /> }, "Layout Panel Left"),
        (html! { <LayoutPanelTop /> }, "Layout Panel Top"),
        (html! { <LayoutTemplate /> }, "Layout Template"),
        (html! { <Leaf /> }, "Leaf"),
        (html! { <LeafyGreen /> }, "Leafy Green"),
        (html! { <Lectern /> }, "Lectern"),
        (html! { <LetterText /> }, "Letter Text"),
        (html! { <LibraryBig /> }, "Library Big"),
        (html! { <Library /> }, "Library"),
        (html! { <LifeBuoy /> }, "Life Buoy"),
        (html! { <Ligature /> }, "Ligature"),
        (html! { <LightbulbOff /> }, "Lightbulb Off"),
        (html! { <Lightbulb /> }, "Lightbulb"),
        (html! { <Link2Off /> }, "Link 2 Off"),
        (html! { <Link2 /> }, "Link 2"),
        (html! { <Link /> }, "Link"),
        (html! { <Linkedin /> }, "Linkedin"),
        (html! { <ListCheck /> }, "List Check"),
        (html! { <ListChecks /> }, "List Checks"),
        (html! { <ListCollapse /> }, "List Collapse"),
        (html! { <ListEnd /> }, "List End"),
        (html! { <ListFilter /> }, "List Filter"),
        (html! { <ListMinus /> }, "List Minus"),
        (html! { <ListMusic /> }, "List Music"),
        (html! { <ListOrdered /> }, "List Ordered"),
        (html! { <ListPlus /> }, "List Plus"),
        (html! { <ListRestart /> }, "List Restart"),
        (html! { <ListStart /> }, "List Start"),
        (html! { <ListTodo /> }, "List Todo"),
        (html! { <ListTree /> }, "List Tree"),
        (html! { <ListVideo /> }, "List Video"),
        (html! { <ListX /> }, "List X"),
        (html! { <List /> }, "List"),
        (html! { <LoaderCircle /> }, "Loader Circle"),
        (html! { <LoaderPinwheel /> }, "Loader Pinwheel"),
        (html! { <Loader /> }, "Loader"),
        (html! { <LocateFixed /> }, "Locate Fixed"),
        (html! { <LocateOff /> }, "Locate Off"),
        (html! { <Locate /> }, "Locate"),
        (html! { <LockKeyholeOpen /> }, "Lock Keyhole Open"),
        (html! { <LockKeyhole /> }, "Lock Keyhole"),
        (html! { <LockOpen /> }, "Lock Open"),
        (html! { <Lock /> }, "Lock"),
        (html! { <LogIn /> }, "Log In"),
        (html! { <LogOut /> }, "Log Out"),
        (html! { <Logs /> }, "Logs"),
        (html! { <Lollipop /> }, "Lollipop"),
        (html! { <Luggage /> }, "Luggage"),
    ];
    icons
        .into_iter()
        .map(|(icon, name)| {
            html! {
                <div class="flex flex-wrap items-center gap-4 text-sm">
                    { icon }
                    <span>{ name }</span>
                </div>
            }
        })
        .collect::<Html>()
}
#[function_component]
pub fn IconsM() -> Html {
    let icons = [
        (html! { <Magnet /> }, "Magnet"),
        (html! { <MailCheck /> }, "Mail Check"),
        (html! { <MailMinus /> }, "Mail Minus"),
        (html! { <MailOpen /> }, "Mail Open"),
        (html! { <MailPlus /> }, "Mail Plus"),
        (html! { <MailQuestion /> }, "Mail Question"),
        (html! { <MailSearch /> }, "Mail Search"),
        (html! { <MailWarning /> }, "Mail Warning"),
        (html! { <MailX /> }, "Mail X"),
        (html! { <Mail /> }, "Mail"),
        (html! { <Mailbox /> }, "Mailbox"),
        (html! { <Mails /> }, "Mails"),
        (html! { <MapPinCheckInside /> }, "Map Pin Check Inside"),
        (html! { <MapPinCheck /> }, "Map Pin Check"),
        (html! { <MapPinHouse /> }, "Map Pin House"),
        (html! { <MapPinMinusInside /> }, "Map Pin Minus Inside"),
        (html! { <MapPinMinus /> }, "Map Pin Minus"),
        (html! { <MapPinOff /> }, "Map Pin Off"),
        (html! { <MapPinPlusInside /> }, "Map Pin Plus Inside"),
        (html! { <MapPinPlus /> }, "Map Pin Plus"),
        (html! { <MapPinXInside /> }, "Map Pin X Inside"),
        (html! { <MapPinX /> }, "Map Pin X"),
        (html! { <MapPin /> }, "Map Pin"),
        (html! { <MapPinned /> }, "Map Pinned"),
        (html! { <Map /> }, "Map"),
        (html! { <Martini /> }, "Martini"),
        (html! { <Maximize2 /> }, "Maximize 2"),
        (html! { <Maximize /> }, "Maximize"),
        (html! { <Medal /> }, "Medal"),
        (html! { <MegaphoneOff /> }, "Megaphone Off"),
        (html! { <Megaphone /> }, "Megaphone"),
        (html! { <Meh /> }, "Meh"),
        (html! { <MemoryStick /> }, "Memory Stick"),
        (html! { <Menu /> }, "Menu"),
        (html! { <Merge /> }, "Merge"),
        (html! { <MessageCircleCode /> }, "Message Circle Code"),
        (html! { <MessageCircleDashed /> }, "Message Circle Dashed"),
        (html! { <MessageCircleHeart /> }, "Message Circle Heart"),
        (html! { <MessageCircleMore /> }, "Message Circle More"),
        (html! { <MessageCircleOff /> }, "Message Circle Off"),
        (html! { <MessageCirclePlus /> }, "Message Circle Plus"),
        (
            html! { <MessageCircleQuestion /> },
            "Message Circle Question",
        ),
        (html! { <MessageCircleReply /> }, "Message Circle Reply"),
        (html! { <MessageCircleWarning /> }, "Message Circle Warning"),
        (html! { <MessageCircleX /> }, "Message Circle X"),
        (html! { <MessageCircle /> }, "Message Circle"),
        (html! { <MessageSquareCode /> }, "Message Square Code"),
        (html! { <MessageSquareDashed /> }, "Message Square Dashed"),
        (html! { <MessageSquareDiff /> }, "Message Square Diff"),
        (html! { <MessageSquareDot /> }, "Message Square Dot"),
        (html! { <MessageSquareHeart /> }, "Message Square Heart"),
        (html! { <MessageSquareLock /> }, "Message Square Lock"),
        (html! { <MessageSquareMore /> }, "Message Square More"),
        (html! { <MessageSquareOff /> }, "Message Square Off"),
        (html! { <MessageSquarePlus /> }, "Message Square Plus"),
        (html! { <MessageSquareQuote /> }, "Message Square Quote"),
        (html! { <MessageSquareReply /> }, "Message Square Reply"),
        (html! { <MessageSquareShare /> }, "Message Square Share"),
        (html! { <MessageSquareText /> }, "Message Square Text"),
        (html! { <MessageSquareWarning /> }, "Message Square Warning"),
        (html! { <MessageSquareX /> }, "Message Square X"),
        (html! { <MessageSquare /> }, "Message Square"),
        (html! { <MessagesSquare /> }, "Messages Square"),
        (html! { <MicOff /> }, "Mic Off"),
        (html! { <MicVocal /> }, "Mic Vocal"),
        (html! { <Mic /> }, "Mic"),
        (html! { <Microchip /> }, "Microchip"),
        (html! { <Microscope /> }, "Microscope"),
        (html! { <Microwave /> }, "Microwave"),
        (html! { <Milestone /> }, "Milestone"),
        (html! { <MilkOff /> }, "Milk Off"),
        (html! { <Milk /> }, "Milk"),
        (html! { <Minimize2 /> }, "Minimize 2"),
        (html! { <Minimize /> }, "Minimize"),
        (html! { <Minus /> }, "Minus"),
        (html! { <MonitorCheck /> }, "Monitor Check"),
        (html! { <MonitorCog /> }, "Monitor Cog"),
        (html! { <MonitorDot /> }, "Monitor Dot"),
        (html! { <MonitorDown /> }, "Monitor Down"),
        (html! { <MonitorOff /> }, "Monitor Off"),
        (html! { <MonitorPause /> }, "Monitor Pause"),
        (html! { <MonitorPlay /> }, "Monitor Play"),
        (html! { <MonitorSmartphone /> }, "Monitor Smartphone"),
        (html! { <MonitorSpeaker /> }, "Monitor Speaker"),
        (html! { <MonitorStop /> }, "Monitor Stop"),
        (html! { <MonitorUp /> }, "Monitor Up"),
        (html! { <MonitorX /> }, "Monitor X"),
        (html! { <Monitor /> }, "Monitor"),
        (html! { <MoonStar /> }, "Moon Star"),
        (html! { <Moon /> }, "Moon"),
        (html! { <MountainSnow /> }, "Mountain Snow"),
        (html! { <Mountain /> }, "Mountain"),
        (html! { <MouseOff /> }, "Mouse Off"),
        (html! { <MousePointer2 /> }, "Mouse Pointer 2"),
        (html! { <MousePointerBan /> }, "Mouse Pointer Ban"),
        (html! { <MousePointerClick /> }, "Mouse Pointer Click"),
        (html! { <MousePointer /> }, "Mouse Pointer"),
        (html! { <Mouse /> }, "Mouse"),
        (html! { <Move3D /> }, "Move 3 D"),
        (html! { <MoveDiagonal2 /> }, "Move Diagonal 2"),
        (html! { <MoveDiagonal /> }, "Move Diagonal"),
        (html! { <MoveDownLeft /> }, "Move Down Left"),
        (html! { <MoveDownRight /> }, "Move Down Right"),
        (html! { <MoveDown /> }, "Move Down"),
        (html! { <MoveHorizontal /> }, "Move Horizontal"),
        (html! { <MoveLeft /> }, "Move Left"),
        (html! { <MoveRight /> }, "Move Right"),
        (html! { <MoveUpLeft /> }, "Move Up Left"),
        (html! { <MoveUpRight /> }, "Move Up Right"),
        (html! { <MoveUp /> }, "Move Up"),
        (html! { <MoveVertical /> }, "Move Vertical"),
        (html! { <Move /> }, "Move"),
        (html! { <Music2 /> }, "Music 2"),
        (html! { <Music3 /> }, "Music 3"),
        (html! { <Music4 /> }, "Music 4"),
        (html! { <Music /> }, "Music"),
    ];
    icons
        .into_iter()
        .map(|(icon, name)| {
            html! {
                <div class="flex flex-wrap items-center gap-4 text-sm">
                    { icon }
                    <span>{ name }</span>
                </div>
            }
        })
        .collect::<Html>()
}
#[function_component]
pub fn IconsN() -> Html {
    let icons = [
        (html! { <Navigation2Off /> }, "Navigation 2 Off"),
        (html! { <Navigation2 /> }, "Navigation 2"),
        (html! { <NavigationOff /> }, "Navigation Off"),
        (html! { <Navigation /> }, "Navigation"),
        (html! { <Network /> }, "Network"),
        (html! { <Newspaper /> }, "Newspaper"),
        (html! { <Nfc /> }, "Nfc"),
        (html! { <NotebookPen /> }, "Notebook Pen"),
        (html! { <NotebookTabs /> }, "Notebook Tabs"),
        (html! { <NotebookText /> }, "Notebook Text"),
        (html! { <Notebook /> }, "Notebook"),
        (html! { <NotepadTextDashed /> }, "Notepad Text Dashed"),
        (html! { <NotepadText /> }, "Notepad Text"),
        (html! { <NutOff /> }, "Nut Off"),
        (html! { <Nut /> }, "Nut"),
    ];
    icons
        .into_iter()
        .map(|(icon, name)| {
            html! {
                <div class="flex flex-wrap items-center gap-4 text-sm">
                    { icon }
                    <span>{ name }</span>
                </div>
            }
        })
        .collect::<Html>()
}
#[function_component]
pub fn IconsO() -> Html {
    let icons = [
        (html! { <OctagonAlert /> }, "Octagon Alert"),
        (html! { <OctagonMinus /> }, "Octagon Minus"),
        (html! { <OctagonPause /> }, "Octagon Pause"),
        (html! { <OctagonX /> }, "Octagon X"),
        (html! { <Octagon /> }, "Octagon"),
        (html! { <Omega /> }, "Omega"),
        (html! { <Option /> }, "Option"),
        (html! { <Orbit /> }, "Orbit"),
        (html! { <Origami /> }, "Origami"),
    ];
    icons
        .into_iter()
        .map(|(icon, name)| {
            html! {
                <div class="flex flex-wrap items-center gap-4 text-sm">
                    { icon }
                    <span>{ name }</span>
                </div>
            }
        })
        .collect::<Html>()
}
#[function_component]
pub fn IconsP() -> Html {
    let icons = [
        (html! { <Package2 /> }, "Package 2"),
        (html! { <PackageCheck /> }, "Package Check"),
        (html! { <PackageMinus /> }, "Package Minus"),
        (html! { <PackageOpen /> }, "Package Open"),
        (html! { <PackagePlus /> }, "Package Plus"),
        (html! { <PackageSearch /> }, "Package Search"),
        (html! { <PackageX /> }, "Package X"),
        (html! { <Package /> }, "Package"),
        (html! { <PaintBucket /> }, "Paint Bucket"),
        (html! { <PaintRoller /> }, "Paint Roller"),
        (html! { <PaintbrushVertical /> }, "Paintbrush Vertical"),
        (html! { <Paintbrush /> }, "Paintbrush"),
        (html! { <Palette /> }, "Palette"),
        (html! { <PanelBottomClose /> }, "Panel Bottom Close"),
        (html! { <PanelBottomDashed /> }, "Panel Bottom Dashed"),
        (html! { <PanelBottomOpen /> }, "Panel Bottom Open"),
        (html! { <PanelBottom /> }, "Panel Bottom"),
        (html! { <PanelLeftClose /> }, "Panel Left Close"),
        (html! { <PanelLeftDashed /> }, "Panel Left Dashed"),
        (html! { <PanelLeftOpen /> }, "Panel Left Open"),
        (html! { <PanelLeft /> }, "Panel Left"),
        (html! { <PanelRightClose /> }, "Panel Right Close"),
        (html! { <PanelRightDashed /> }, "Panel Right Dashed"),
        (html! { <PanelRightOpen /> }, "Panel Right Open"),
        (html! { <PanelRight /> }, "Panel Right"),
        (html! { <PanelTopClose /> }, "Panel Top Close"),
        (html! { <PanelTopDashed /> }, "Panel Top Dashed"),
        (html! { <PanelTopOpen /> }, "Panel Top Open"),
        (html! { <PanelTop /> }, "Panel Top"),
        (html! { <PanelsLeftBottom /> }, "Panels Left Bottom"),
        (html! { <PanelsRightBottom /> }, "Panels Right Bottom"),
        (html! { <PanelsTopLeft /> }, "Panels Top Left"),
        (html! { <Paperclip /> }, "Paperclip"),
        (html! { <Parentheses /> }, "Parentheses"),
        (html! { <ParkingMeter /> }, "Parking Meter"),
        (html! { <PartyPopper /> }, "Party Popper"),
        (html! { <Pause /> }, "Pause"),
        (html! { <PawPrint /> }, "Paw Print"),
        (html! { <PcCase /> }, "Pc Case"),
        (html! { <PenLine /> }, "Pen Line"),
        (html! { <PenOff /> }, "Pen Off"),
        (html! { <PenTool /> }, "Pen Tool"),
        (html! { <Pen /> }, "Pen"),
        (html! { <PencilLine /> }, "Pencil Line"),
        (html! { <PencilOff /> }, "Pencil Off"),
        (html! { <PencilRuler /> }, "Pencil Ruler"),
        (html! { <Pencil /> }, "Pencil"),
        (html! { <Pentagon /> }, "Pentagon"),
        (html! { <Percent /> }, "Percent"),
        (html! { <PersonStanding /> }, "Person Standing"),
        (html! { <PhilippinePeso /> }, "Philippine Peso"),
        (html! { <PhoneCall /> }, "Phone Call"),
        (html! { <PhoneForwarded /> }, "Phone Forwarded"),
        (html! { <PhoneIncoming /> }, "Phone Incoming"),
        (html! { <PhoneMissed /> }, "Phone Missed"),
        (html! { <PhoneOff /> }, "Phone Off"),
        (html! { <PhoneOutgoing /> }, "Phone Outgoing"),
        (html! { <Phone /> }, "Phone"),
        (html! { <Pi /> }, "Pi"),
        (html! { <Piano /> }, "Piano"),
        (html! { <Pickaxe /> }, "Pickaxe"),
        (html! { <PictureInPicture2 /> }, "Picture In Picture 2"),
        (html! { <PictureInPicture /> }, "Picture In Picture"),
        (html! { <PiggyBank /> }, "Piggy Bank"),
        (html! { <PilcrowLeft /> }, "Pilcrow Left"),
        (html! { <PilcrowRight /> }, "Pilcrow Right"),
        (html! { <Pilcrow /> }, "Pilcrow"),
        (html! { <PillBottle /> }, "Pill Bottle"),
        (html! { <Pill /> }, "Pill"),
        (html! { <PinOff /> }, "Pin Off"),
        (html! { <Pin /> }, "Pin"),
        (html! { <Pipette /> }, "Pipette"),
        (html! { <Pizza /> }, "Pizza"),
        (html! { <PlaneLanding /> }, "Plane Landing"),
        (html! { <PlaneTakeoff /> }, "Plane Takeoff"),
        (html! { <Plane /> }, "Plane"),
        (html! { <Play /> }, "Play"),
        (html! { <Plug2 /> }, "Plug 2"),
        (html! { <PlugZap /> }, "Plug Zap"),
        (html! { <Plug /> }, "Plug"),
        (html! { <Plus /> }, "Plus"),
        (html! { <PocketKnife /> }, "Pocket Knife"),
        (html! { <Pocket /> }, "Pocket"),
        (html! { <Podcast /> }, "Podcast"),
        (html! { <PointerOff /> }, "Pointer Off"),
        (html! { <Pointer /> }, "Pointer"),
        (html! { <Popcorn /> }, "Popcorn"),
        (html! { <Popsicle /> }, "Popsicle"),
        (html! { <PoundSterling /> }, "Pound Sterling"),
        (html! { <PowerOff /> }, "Power Off"),
        (html! { <Power /> }, "Power"),
        (html! { <Presentation /> }, "Presentation"),
        (html! { <PrinterCheck /> }, "Printer Check"),
        (html! { <Printer /> }, "Printer"),
        (html! { <Projector /> }, "Projector"),
        (html! { <Proportions /> }, "Proportions"),
        (html! { <Puzzle /> }, "Puzzle"),
        (html! { <Pyramid /> }, "Pyramid"),
    ];
    icons
        .into_iter()
        .map(|(icon, name)| {
            html! {
                <div class="flex flex-wrap items-center gap-4 text-sm">
                    { icon }
                    <span>{ name }</span>
                </div>
            }
        })
        .collect::<Html>()
}
#[function_component]
pub fn IconsQ() -> Html {
    let icons = [
        (html! { <QrCode /> }, "Qr Code"),
        (html! { <Quote /> }, "Quote"),
    ];
    icons
        .into_iter()
        .map(|(icon, name)| {
            html! {
                <div class="flex flex-wrap items-center gap-4 text-sm">
                    { icon }
                    <span>{ name }</span>
                </div>
            }
        })
        .collect::<Html>()
}
#[function_component]
pub fn IconsR() -> Html {
    let icons = [
        (html! { <Rabbit /> }, "Rabbit"),
        (html! { <Radar /> }, "Radar"),
        (html! { <Radiation /> }, "Radiation"),
        (html! { <Radical /> }, "Radical"),
        (html! { <RadioReceiver /> }, "Radio Receiver"),
        (html! { <RadioTower /> }, "Radio Tower"),
        (html! { <Radio /> }, "Radio"),
        (html! { <Radius /> }, "Radius"),
        (html! { <RailSymbol /> }, "Rail Symbol"),
        (html! { <Rainbow /> }, "Rainbow"),
        (html! { <Rat /> }, "Rat"),
        (html! { <Ratio /> }, "Ratio"),
        (html! { <ReceiptCent /> }, "Receipt Cent"),
        (html! { <ReceiptEuro /> }, "Receipt Euro"),
        (html! { <ReceiptIndianRupee /> }, "Receipt Indian Rupee"),
        (html! { <ReceiptJapaneseYen /> }, "Receipt Japanese Yen"),
        (html! { <ReceiptPoundSterling /> }, "Receipt Pound Sterling"),
        (html! { <ReceiptRussianRuble /> }, "Receipt Russian Ruble"),
        (html! { <ReceiptSwissFranc /> }, "Receipt Swiss Franc"),
        (html! { <ReceiptText /> }, "Receipt Text"),
        (html! { <Receipt /> }, "Receipt"),
        (html! { <RectangleEllipsis /> }, "Rectangle Ellipsis"),
        (html! { <RectangleHorizontal /> }, "Rectangle Horizontal"),
        (html! { <RectangleVertical /> }, "Rectangle Vertical"),
        (html! { <Recycle /> }, "Recycle"),
        (html! { <Redo2 /> }, "Redo 2"),
        (html! { <RedoDot /> }, "Redo Dot"),
        (html! { <Redo /> }, "Redo"),
        (html! { <RefreshCcwDot /> }, "Refresh Ccw Dot"),
        (html! { <RefreshCcw /> }, "Refresh Ccw"),
        (html! { <RefreshCwOff /> }, "Refresh Cw Off"),
        (html! { <RefreshCw /> }, "Refresh Cw"),
        (html! { <Refrigerator /> }, "Refrigerator"),
        (html! { <Regex /> }, "Regex"),
        (html! { <RemoveFormatting /> }, "Remove Formatting"),
        (html! { <Repeat1 /> }, "Repeat 1"),
        (html! { <Repeat2 /> }, "Repeat 2"),
        (html! { <Repeat /> }, "Repeat"),
        (html! { <ReplaceAll /> }, "Replace All"),
        (html! { <Replace /> }, "Replace"),
        (html! { <ReplyAll /> }, "Reply All"),
        (html! { <Reply /> }, "Reply"),
        (html! { <Rewind /> }, "Rewind"),
        (html! { <Ribbon /> }, "Ribbon"),
        (html! { <Rocket /> }, "Rocket"),
        (html! { <RockingChair /> }, "Rocking Chair"),
        (html! { <RollerCoaster /> }, "Roller Coaster"),
        (html! { <Rotate3D /> }, "Rotate 3 D"),
        (html! { <RotateCcwSquare /> }, "Rotate Ccw Square"),
        (html! { <RotateCcw /> }, "Rotate Ccw"),
        (html! { <RotateCwSquare /> }, "Rotate Cw Square"),
        (html! { <RotateCw /> }, "Rotate Cw"),
        (html! { <RouteOff /> }, "Route Off"),
        (html! { <Route /> }, "Route"),
        (html! { <Router /> }, "Router"),
        (html! { <Rows2 /> }, "Rows 2"),
        (html! { <Rows3 /> }, "Rows 3"),
        (html! { <Rows4 /> }, "Rows 4"),
        (html! { <Rss /> }, "Rss"),
        (html! { <Ruler /> }, "Ruler"),
        (html! { <RussianRuble /> }, "Russian Ruble"),
    ];
    icons
        .into_iter()
        .map(|(icon, name)| {
            html! {
                <div class="flex flex-wrap items-center gap-4 text-sm">
                    { icon }
                    <span>{ name }</span>
                </div>
            }
        })
        .collect::<Html>()
}
#[function_component]
pub fn IconsS() -> Html {
    let icons = [
        (html! { <Sailboat /> }, "Sailboat"),
        (html! { <Salad /> }, "Salad"),
        (html! { <Sandwich /> }, "Sandwich"),
        (html! { <SatelliteDish /> }, "Satellite Dish"),
        (html! { <Satellite /> }, "Satellite"),
        (html! { <SaveAll /> }, "Save All"),
        (html! { <SaveOff /> }, "Save Off"),
        (html! { <Save /> }, "Save"),
        (html! { <Scale3D /> }, "Scale 3 D"),
        (html! { <Scale /> }, "Scale"),
        (html! { <Scaling /> }, "Scaling"),
        (html! { <ScanBarcode /> }, "Scan Barcode"),
        (html! { <ScanEye /> }, "Scan Eye"),
        (html! { <ScanFace /> }, "Scan Face"),
        (html! { <ScanLine /> }, "Scan Line"),
        (html! { <ScanQrCode /> }, "Scan Qr Code"),
        (html! { <ScanSearch /> }, "Scan Search"),
        (html! { <ScanText /> }, "Scan Text"),
        (html! { <Scan /> }, "Scan"),
        (html! { <School /> }, "School"),
        (html! { <ScissorsLineDashed /> }, "Scissors Line Dashed"),
        (html! { <Scissors /> }, "Scissors"),
        (html! { <ScreenShareOff /> }, "Screen Share Off"),
        (html! { <ScreenShare /> }, "Screen Share"),
        (html! { <ScrollText /> }, "Scroll Text"),
        (html! { <Scroll /> }, "Scroll"),
        (html! { <SearchCheck /> }, "Search Check"),
        (html! { <SearchCode /> }, "Search Code"),
        (html! { <SearchSlash /> }, "Search Slash"),
        (html! { <SearchX /> }, "Search X"),
        (html! { <Search /> }, "Search"),
        (html! { <Section /> }, "Section"),
        (html! { <SendHorizontal /> }, "Send Horizontal"),
        (html! { <SendToBack /> }, "Send To Back"),
        (html! { <Send /> }, "Send"),
        (html! { <SeparatorHorizontal /> }, "Separator Horizontal"),
        (html! { <SeparatorVertical /> }, "Separator Vertical"),
        (html! { <ServerCog /> }, "Server Cog"),
        (html! { <ServerCrash /> }, "Server Crash"),
        (html! { <ServerOff /> }, "Server Off"),
        (html! { <Server /> }, "Server"),
        (html! { <Settings2 /> }, "Settings 2"),
        (html! { <Settings /> }, "Settings"),
        (html! { <Shapes /> }, "Shapes"),
        (html! { <Share2 /> }, "Share 2"),
        (html! { <Share /> }, "Share"),
        (html! { <Sheet /> }, "Sheet"),
        (html! { <Shell /> }, "Shell"),
        (html! { <ShieldAlert /> }, "Shield Alert"),
        (html! { <ShieldBan /> }, "Shield Ban"),
        (html! { <ShieldCheck /> }, "Shield Check"),
        (html! { <ShieldEllipsis /> }, "Shield Ellipsis"),
        (html! { <ShieldHalf /> }, "Shield Half"),
        (html! { <ShieldMinus /> }, "Shield Minus"),
        (html! { <ShieldOff /> }, "Shield Off"),
        (html! { <ShieldPlus /> }, "Shield Plus"),
        (html! { <ShieldQuestion /> }, "Shield Question"),
        (html! { <ShieldX /> }, "Shield X"),
        (html! { <Shield /> }, "Shield"),
        (html! { <ShipWheel /> }, "Ship Wheel"),
        (html! { <Ship /> }, "Ship"),
        (html! { <Shirt /> }, "Shirt"),
        (html! { <ShoppingBag /> }, "Shopping Bag"),
        (html! { <ShoppingBasket /> }, "Shopping Basket"),
        (html! { <ShoppingCart /> }, "Shopping Cart"),
        (html! { <Shovel /> }, "Shovel"),
        (html! { <ShowerHead /> }, "Shower Head"),
        (html! { <Shrink /> }, "Shrink"),
        (html! { <Shrub /> }, "Shrub"),
        (html! { <Shuffle /> }, "Shuffle"),
        (html! { <Sigma /> }, "Sigma"),
        (html! { <SignalHigh /> }, "Signal High"),
        (html! { <SignalLow /> }, "Signal Low"),
        (html! { <SignalMedium /> }, "Signal Medium"),
        (html! { <SignalZero /> }, "Signal Zero"),
        (html! { <Signal /> }, "Signal"),
        (html! { <Signature /> }, "Signature"),
        (html! { <SignpostBig /> }, "Signpost Big"),
        (html! { <Signpost /> }, "Signpost"),
        (html! { <Siren /> }, "Siren"),
        (html! { <SkipBack /> }, "Skip Back"),
        (html! { <SkipForward /> }, "Skip Forward"),
        (html! { <Skull /> }, "Skull"),
        (html! { <Slack /> }, "Slack"),
        (html! { <Slash /> }, "Slash"),
        (html! { <Slice /> }, "Slice"),
        (html! { <SlidersHorizontal /> }, "Sliders Horizontal"),
        (html! { <SlidersVertical /> }, "Sliders Vertical"),
        (html! { <SmartphoneCharging /> }, "Smartphone Charging"),
        (html! { <SmartphoneNfc /> }, "Smartphone Nfc"),
        (html! { <Smartphone /> }, "Smartphone"),
        (html! { <SmilePlus /> }, "Smile Plus"),
        (html! { <Smile /> }, "Smile"),
        (html! { <Snail /> }, "Snail"),
        (html! { <Snowflake /> }, "Snowflake"),
        (html! { <Sofa /> }, "Sofa"),
        (html! { <Soup /> }, "Soup"),
        (html! { <Space /> }, "Space"),
        (html! { <Spade /> }, "Spade"),
        (html! { <Sparkle /> }, "Sparkle"),
        (html! { <Sparkles /> }, "Sparkles"),
        (html! { <Speaker /> }, "Speaker"),
        (html! { <Speech /> }, "Speech"),
        (html! { <SpellCheck2 /> }, "Spell Check 2"),
        (html! { <SpellCheck /> }, "Spell Check"),
        (html! { <Spline /> }, "Spline"),
        (html! { <Split /> }, "Split"),
        (html! { <SprayCan /> }, "Spray Can"),
        (html! { <Sprout /> }, "Sprout"),
        (html! { <SquareActivity /> }, "Square Activity"),
        (html! { <SquareArrowDownLeft /> }, "Square Arrow Down Left"),
        (
            html! { <SquareArrowDownRight /> },
            "Square Arrow Down Right",
        ),
        (html! { <SquareArrowDown /> }, "Square Arrow Down"),
        (html! { <SquareArrowLeft /> }, "Square Arrow Left"),
        (
            html! { <SquareArrowOutDownLeft /> },
            "Square Arrow Out Down Left",
        ),
        (
            html! { <SquareArrowOutDownRight /> },
            "Square Arrow Out Down Right",
        ),
        (
            html! { <SquareArrowOutUpLeft /> },
            "Square Arrow Out Up Left",
        ),
        (
            html! { <SquareArrowOutUpRight /> },
            "Square Arrow Out Up Right",
        ),
        (html! { <SquareArrowRight /> }, "Square Arrow Right"),
        (html! { <SquareArrowUpLeft /> }, "Square Arrow Up Left"),
        (html! { <SquareArrowUpRight /> }, "Square Arrow Up Right"),
        (html! { <SquareArrowUp /> }, "Square Arrow Up"),
        (html! { <SquareAsterisk /> }, "Square Asterisk"),
        (
            html! { <SquareBottomDashedScissors /> },
            "Square Bottom Dashed Scissors",
        ),
        (html! { <SquareChartGantt /> }, "Square Chart Gantt"),
        (html! { <SquareCheckBig /> }, "Square Check Big"),
        (html! { <SquareCheck /> }, "Square Check"),
        (html! { <SquareChevronDown /> }, "Square Chevron Down"),
        (html! { <SquareChevronLeft /> }, "Square Chevron Left"),
        (html! { <SquareChevronRight /> }, "Square Chevron Right"),
        (html! { <SquareChevronUp /> }, "Square Chevron Up"),
        (html! { <SquareCode /> }, "Square Code"),
        (
            html! { <SquareDashedBottomCode /> },
            "Square Dashed Bottom Code",
        ),
        (html! { <SquareDashedBottom /> }, "Square Dashed Bottom"),
        (html! { <SquareDashedKanban /> }, "Square Dashed Kanban"),
        (
            html! { <SquareDashedMousePointer /> },
            "Square Dashed Mouse Pointer",
        ),
        (html! { <SquareDashed /> }, "Square Dashed"),
        (html! { <SquareDivide /> }, "Square Divide"),
        (html! { <SquareDot /> }, "Square Dot"),
        (html! { <SquareEqual /> }, "Square Equal"),
        (html! { <SquareFunction /> }, "Square Function"),
        (html! { <SquareKanban /> }, "Square Kanban"),
        (html! { <SquareLibrary /> }, "Square Library"),
        (html! { <SquareM /> }, "Square M"),
        (html! { <SquareMenu /> }, "Square Menu"),
        (html! { <SquareMinus /> }, "Square Minus"),
        (html! { <SquareMousePointer /> }, "Square Mouse Pointer"),
        (html! { <SquareParkingOff /> }, "Square Parking Off"),
        (html! { <SquareParking /> }, "Square Parking"),
        (html! { <SquarePen /> }, "Square Pen"),
        (html! { <SquarePercent /> }, "Square Percent"),
        (html! { <SquarePi /> }, "Square Pi"),
        (html! { <SquarePilcrow /> }, "Square Pilcrow"),
        (html! { <SquarePlay /> }, "Square Play"),
        (html! { <SquarePlus /> }, "Square Plus"),
        (html! { <SquarePower /> }, "Square Power"),
        (html! { <SquareRadical /> }, "Square Radical"),
        (html! { <SquareScissors /> }, "Square Scissors"),
        (html! { <SquareSigma /> }, "Square Sigma"),
        (html! { <SquareSlash /> }, "Square Slash"),
        (
            html! { <SquareSplitHorizontal /> },
            "Square Split Horizontal",
        ),
        (html! { <SquareSplitVertical /> }, "Square Split Vertical"),
        (html! { <SquareSquare /> }, "Square Square"),
        (html! { <SquareStack /> }, "Square Stack"),
        (html! { <SquareTerminal /> }, "Square Terminal"),
        (html! { <SquareUserRound /> }, "Square User Round"),
        (html! { <SquareUser /> }, "Square User"),
        (html! { <SquareX /> }, "Square X"),
        (html! { <Square /> }, "Square"),
        (html! { <Squircle /> }, "Squircle"),
        (html! { <Squirrel /> }, "Squirrel"),
        (html! { <Stamp /> }, "Stamp"),
        (html! { <StarHalf /> }, "Star Half"),
        (html! { <StarOff /> }, "Star Off"),
        (html! { <Star /> }, "Star"),
        (html! { <StepBack /> }, "Step Back"),
        (html! { <StepForward /> }, "Step Forward"),
        (html! { <Stethoscope /> }, "Stethoscope"),
        (html! { <Sticker /> }, "Sticker"),
        (html! { <StickyNote /> }, "Sticky Note"),
        (html! { <Store /> }, "Store"),
        (html! { <StretchHorizontal /> }, "Stretch Horizontal"),
        (html! { <StretchVertical /> }, "Stretch Vertical"),
        (html! { <Strikethrough /> }, "Strikethrough"),
        (html! { <Subscript /> }, "Subscript"),
        (html! { <SunDim /> }, "Sun Dim"),
        (html! { <SunMedium /> }, "Sun Medium"),
        (html! { <SunMoon /> }, "Sun Moon"),
        (html! { <SunSnow /> }, "Sun Snow"),
        (html! { <Sun /> }, "Sun"),
        (html! { <Sunrise /> }, "Sunrise"),
        (html! { <Sunset /> }, "Sunset"),
        (html! { <Superscript /> }, "Superscript"),
        (html! { <SwatchBook /> }, "Swatch Book"),
        (html! { <SwissFranc /> }, "Swiss Franc"),
        (html! { <SwitchCamera /> }, "Switch Camera"),
        (html! { <Sword /> }, "Sword"),
        (html! { <Swords /> }, "Swords"),
        (html! { <Syringe /> }, "Syringe"),
    ];
    icons
        .into_iter()
        .map(|(icon, name)| {
            html! {
                <div class="flex flex-wrap items-center gap-4 text-sm">
                    { icon }
                    <span>{ name }</span>
                </div>
            }
        })
        .collect::<Html>()
}
#[function_component]
pub fn IconsT() -> Html {
    let icons = [
        (html! { <Table2 /> }, "Table 2"),
        (html! { <TableCellsMerge /> }, "Table Cells Merge"),
        (html! { <TableCellsSplit /> }, "Table Cells Split"),
        (html! { <TableColumnsSplit /> }, "Table Columns Split"),
        (html! { <TableOfContents /> }, "Table Of Contents"),
        (html! { <TableProperties /> }, "Table Properties"),
        (html! { <TableRowsSplit /> }, "Table Rows Split"),
        (html! { <Table /> }, "Table"),
        (html! { <TabletSmartphone /> }, "Tablet Smartphone"),
        (html! { <Tablet /> }, "Tablet"),
        (html! { <Tablets /> }, "Tablets"),
        (html! { <Tag /> }, "Tag"),
        (html! { <Tags /> }, "Tags"),
        (html! { <Tally1 /> }, "Tally 1"),
        (html! { <Tally2 /> }, "Tally 2"),
        (html! { <Tally3 /> }, "Tally 3"),
        (html! { <Tally4 /> }, "Tally 4"),
        (html! { <Tally5 /> }, "Tally 5"),
        (html! { <Tangent /> }, "Tangent"),
        (html! { <Target /> }, "Target"),
        (html! { <Telescope /> }, "Telescope"),
        (html! { <TentTree /> }, "Tent Tree"),
        (html! { <Tent /> }, "Tent"),
        (html! { <Terminal /> }, "Terminal"),
        (html! { <TestTubeDiagonal /> }, "Test Tube Diagonal"),
        (html! { <TestTube /> }, "Test Tube"),
        (html! { <TestTubes /> }, "Test Tubes"),
        (html! { <TextCursorInput /> }, "Text Cursor Input"),
        (html! { <TextCursor /> }, "Text Cursor"),
        (html! { <TextQuote /> }, "Text Quote"),
        (html! { <TextSearch /> }, "Text Search"),
        (html! { <TextSelect /> }, "Text Select"),
        (html! { <Text /> }, "Text"),
        (html! { <Theater /> }, "Theater"),
        (html! { <ThermometerSnowflake /> }, "Thermometer Snowflake"),
        (html! { <ThermometerSun /> }, "Thermometer Sun"),
        (html! { <Thermometer /> }, "Thermometer"),
        (html! { <ThumbsDown /> }, "Thumbs Down"),
        (html! { <ThumbsUp /> }, "Thumbs Up"),
        (html! { <TicketCheck /> }, "Ticket Check"),
        (html! { <TicketMinus /> }, "Ticket Minus"),
        (html! { <TicketPercent /> }, "Ticket Percent"),
        (html! { <TicketPlus /> }, "Ticket Plus"),
        (html! { <TicketSlash /> }, "Ticket Slash"),
        (html! { <TicketX /> }, "Ticket X"),
        (html! { <Ticket /> }, "Ticket"),
        (html! { <TicketsPlane /> }, "Tickets Plane"),
        (html! { <Tickets /> }, "Tickets"),
        (html! { <TimerOff /> }, "Timer Off"),
        (html! { <TimerReset /> }, "Timer Reset"),
        (html! { <Timer /> }, "Timer"),
        (html! { <ToggleLeft /> }, "Toggle Left"),
        (html! { <ToggleRight /> }, "Toggle Right"),
        (html! { <Toilet /> }, "Toilet"),
        (html! { <Tornado /> }, "Tornado"),
        (html! { <Torus /> }, "Torus"),
        (html! { <TouchpadOff /> }, "Touchpad Off"),
        (html! { <Touchpad /> }, "Touchpad"),
        (html! { <TowerControl /> }, "Tower Control"),
        (html! { <ToyBrick /> }, "Toy Brick"),
        (html! { <Tractor /> }, "Tractor"),
        (html! { <TrafficCone /> }, "Traffic Cone"),
        (html! { <TrainFrontTunnel /> }, "Train Front Tunnel"),
        (html! { <TrainFront /> }, "Train Front"),
        (html! { <TrainTrack /> }, "Train Track"),
        (html! { <TramFront /> }, "Tram Front"),
        (html! { <Trash2 /> }, "Trash 2"),
        (html! { <Trash /> }, "Trash"),
        (html! { <TreeDeciduous /> }, "Tree Deciduous"),
        (html! { <TreePalm /> }, "Tree Palm"),
        (html! { <TreePine /> }, "Tree Pine"),
        (html! { <Trees /> }, "Trees"),
        (html! { <Trello /> }, "Trello"),
        (html! { <TrendingDown /> }, "Trending Down"),
        (html! { <TrendingUpDown /> }, "Trending Up Down"),
        (html! { <TrendingUp /> }, "Trending Up"),
        (html! { <TriangleAlert /> }, "Triangle Alert"),
        (html! { <TriangleRight /> }, "Triangle Right"),
        (html! { <Triangle /> }, "Triangle"),
        (html! { <Trophy /> }, "Trophy"),
        (html! { <Truck /> }, "Truck"),
        (html! { <Turtle /> }, "Turtle"),
        (html! { <TvMinimalPlay /> }, "Tv Minimal Play"),
        (html! { <TvMinimal /> }, "Tv Minimal"),
        (html! { <Tv /> }, "Tv"),
        (html! { <Twitch /> }, "Twitch"),
        (html! { <Twitter /> }, "Twitter"),
        (html! { <TypeOutline /> }, "Type Outline"),
        (html! { <Type /> }, "Type"),
    ];
    icons
        .into_iter()
        .map(|(icon, name)| {
            html! {
                <div class="flex flex-wrap items-center gap-4 text-sm">
                    { icon }
                    <span>{ name }</span>
                </div>
            }
        })
        .collect::<Html>()
}
#[function_component]
pub fn IconsU() -> Html {
    let icons = [
        (html! { <UmbrellaOff /> }, "Umbrella Off"),
        (html! { <Umbrella /> }, "Umbrella"),
        (html! { <Underline /> }, "Underline"),
        (html! { <Undo2 /> }, "Undo 2"),
        (html! { <UndoDot /> }, "Undo Dot"),
        (html! { <Undo /> }, "Undo"),
        (html! { <UnfoldHorizontal /> }, "Unfold Horizontal"),
        (html! { <UnfoldVertical /> }, "Unfold Vertical"),
        (html! { <Ungroup /> }, "Ungroup"),
        (html! { <University /> }, "University"),
        (html! { <Unlink2 /> }, "Unlink 2"),
        (html! { <Unlink /> }, "Unlink"),
        (html! { <Unplug /> }, "Unplug"),
        (html! { <Upload /> }, "Upload"),
        (html! { <Usb /> }, "Usb"),
        (html! { <UserCheck /> }, "User Check"),
        (html! { <UserCog /> }, "User Cog"),
        (html! { <UserMinus /> }, "User Minus"),
        (html! { <UserPen /> }, "User Pen"),
        (html! { <UserPlus /> }, "User Plus"),
        (html! { <UserRoundCheck /> }, "User Round Check"),
        (html! { <UserRoundCog /> }, "User Round Cog"),
        (html! { <UserRoundMinus /> }, "User Round Minus"),
        (html! { <UserRoundPen /> }, "User Round Pen"),
        (html! { <UserRoundPlus /> }, "User Round Plus"),
        (html! { <UserRoundSearch /> }, "User Round Search"),
        (html! { <UserRoundX /> }, "User Round X"),
        (html! { <UserRound /> }, "User Round"),
        (html! { <UserSearch /> }, "User Search"),
        (html! { <UserX /> }, "User X"),
        (html! { <User /> }, "User"),
        (html! { <UsersRound /> }, "Users Round"),
        (html! { <Users /> }, "Users"),
        (html! { <UtensilsCrossed /> }, "Utensils Crossed"),
        (html! { <Utensils /> }, "Utensils"),
        (html! { <UtilityPole /> }, "Utility Pole"),
    ];
    icons
        .into_iter()
        .map(|(icon, name)| {
            html! {
                <div class="flex flex-wrap items-center gap-4 text-sm">
                    { icon }
                    <span>{ name }</span>
                </div>
            }
        })
        .collect::<Html>()
}
#[function_component]
pub fn IconsV() -> Html {
    let icons = [
        (html! { <Variable /> }, "Variable"),
        (html! { <Vault /> }, "Vault"),
        (html! { <Vegan /> }, "Vegan"),
        (html! { <VenetianMask /> }, "Venetian Mask"),
        (html! { <VibrateOff /> }, "Vibrate Off"),
        (html! { <Vibrate /> }, "Vibrate"),
        (html! { <VideoOff /> }, "Video Off"),
        (html! { <Video /> }, "Video"),
        (html! { <Videotape /> }, "Videotape"),
        (html! { <View /> }, "View"),
        (html! { <Voicemail /> }, "Voicemail"),
        (html! { <Volleyball /> }, "Volleyball"),
        (html! { <Volume1 /> }, "Volume 1"),
        (html! { <Volume2 /> }, "Volume 2"),
        (html! { <VolumeOff /> }, "Volume Off"),
        (html! { <VolumeX /> }, "Volume X"),
        (html! { <Volume /> }, "Volume"),
        (html! { <Vote /> }, "Vote"),
    ];
    icons
        .into_iter()
        .map(|(icon, name)| {
            html! {
                <div class="flex flex-wrap items-center gap-4 text-sm">
                    { icon }
                    <span>{ name }</span>
                </div>
            }
        })
        .collect::<Html>()
}
#[function_component]
pub fn IconsW() -> Html {
    let icons = [
        (html! { <WalletCards /> }, "Wallet Cards"),
        (html! { <WalletMinimal /> }, "Wallet Minimal"),
        (html! { <Wallet /> }, "Wallet"),
        (html! { <Wallpaper /> }, "Wallpaper"),
        (html! { <WandSparkles /> }, "Wand Sparkles"),
        (html! { <Wand /> }, "Wand"),
        (html! { <Warehouse /> }, "Warehouse"),
        (html! { <WashingMachine /> }, "Washing Machine"),
        (html! { <Watch /> }, "Watch"),
        (html! { <Waves /> }, "Waves"),
        (html! { <Waypoints /> }, "Waypoints"),
        (html! { <Webcam /> }, "Webcam"),
        (html! { <WebhookOff /> }, "Webhook Off"),
        (html! { <Webhook /> }, "Webhook"),
        (html! { <Weight /> }, "Weight"),
        (html! { <WheatOff /> }, "Wheat Off"),
        (html! { <Wheat /> }, "Wheat"),
        (html! { <WholeWord /> }, "Whole Word"),
        (html! { <WifiHigh /> }, "Wifi High"),
        (html! { <WifiLow /> }, "Wifi Low"),
        (html! { <WifiOff /> }, "Wifi Off"),
        (html! { <WifiZero /> }, "Wifi Zero"),
        (html! { <Wifi /> }, "Wifi"),
        (html! { <WindArrowDown /> }, "Wind Arrow Down"),
        (html! { <Wind /> }, "Wind"),
        (html! { <WineOff /> }, "Wine Off"),
        (html! { <Wine /> }, "Wine"),
        (html! { <Workflow /> }, "Workflow"),
        (html! { <Worm /> }, "Worm"),
        (html! { <WrapText /> }, "Wrap Text"),
        (html! { <Wrench /> }, "Wrench"),
    ];
    icons
        .into_iter()
        .map(|(icon, name)| {
            html! {
                <div class="flex flex-wrap items-center gap-4 text-sm">
                    { icon }
                    <span>{ name }</span>
                </div>
            }
        })
        .collect::<Html>()
}
#[function_component]
pub fn IconsX() -> Html {
    let icons = [(html! { <X /> }, "X")];
    icons
        .into_iter()
        .map(|(icon, name)| {
            html! {
                <div class="flex flex-wrap items-center gap-4 text-sm">
                    { icon }
                    <span>{ name }</span>
                </div>
            }
        })
        .collect::<Html>()
}
#[function_component]
pub fn IconsY() -> Html {
    let icons = [(html! { <Youtube /> }, "Youtube")];
    icons
        .into_iter()
        .map(|(icon, name)| {
            html! {
                <div class="flex flex-wrap items-center gap-4 text-sm">
                    { icon }
                    <span>{ name }</span>
                </div>
            }
        })
        .collect::<Html>()
}
#[function_component]
pub fn IconsZ() -> Html {
    let icons = [
        (html! { <ZapOff /> }, "Zap Off"),
        (html! { <Zap /> }, "Zap"),
        (html! { <ZoomIn /> }, "Zoom In"),
        (html! { <ZoomOut /> }, "Zoom Out"),
    ];
    icons
        .into_iter()
        .map(|(icon, name)| {
            html! {
                <div class="flex flex-wrap items-center gap-4 text-sm">
                    { icon }
                    <span>{ name }</span>
                </div>
            }
        })
        .collect::<Html>()
}
