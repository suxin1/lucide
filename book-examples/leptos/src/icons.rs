use leptos::prelude::*;
use lucide_leptos::*;
#[component]
pub fn Icons() -> impl IntoView {
    view! {
        <div class="w-full max-w-80 py-4">
            <IconsA/>
            <IconsB/>
            <IconsC/>
            <IconsD/>
            <IconsE/>
            <IconsF/>
            <IconsG/>
            <IconsH/>
            <IconsI/>
            <IconsJ/>
            <IconsK/>
            <IconsL/>
            <IconsM/>
            <IconsN/>
            <IconsO/>
            <IconsP/>
            <IconsQ/>
            <IconsR/>
            <IconsS/>
            <IconsT/>
            <IconsU/>
            <IconsV/>
            <IconsW/>
            <IconsX/>
            <IconsY/>
            <IconsZ/>
        </div>
    }
}
#[component]
pub fn IconsA() -> impl IntoView {
    view! {
        <For
            each=move || [
                (view! { <AArrowDown/> }.into_any(), "A Arrow Down"),
                (view! { <AArrowUp/> }.into_any(), "A Arrow Up"),
                (view! { <ALargeSmall/> }.into_any(), "A Large Small"),
                (view! { <Accessibility/> }.into_any(), "Accessibility"),
                (view! { <Activity/> }.into_any(), "Activity"),
                (view! { <AirVent/> }.into_any(), "Air Vent"),
                (view! { <Airplay/> }.into_any(), "Airplay"),
                (view! { <AlarmClockCheck/> }.into_any(), "Alarm Clock Check"),
                (view! { <AlarmClockMinus/> }.into_any(), "Alarm Clock Minus"),
                (view! { <AlarmClockOff/> }.into_any(), "Alarm Clock Off"),
                (view! { <AlarmClockPlus/> }.into_any(), "Alarm Clock Plus"),
                (view! { <AlarmClock/> }.into_any(), "Alarm Clock"),
                (view! { <AlarmSmoke/> }.into_any(), "Alarm Smoke"),
                (view! { <Album/> }.into_any(), "Album"),
                (view! { <AlignCenterHorizontal/> }.into_any(), "Align Center Horizontal"),
                (view! { <AlignCenterVertical/> }.into_any(), "Align Center Vertical"),
                (view! { <AlignCenter/> }.into_any(), "Align Center"),
                (view! { <AlignEndHorizontal/> }.into_any(), "Align End Horizontal"),
                (view! { <AlignEndVertical/> }.into_any(), "Align End Vertical"),
                (
                    view! { <AlignHorizontalDistributeCenter/> }.into_any(),
                    "Align Horizontal Distribute Center",
                ),
                (
                    view! { <AlignHorizontalDistributeEnd/> }.into_any(),
                    "Align Horizontal Distribute End",
                ),
                (
                    view! { <AlignHorizontalDistributeStart/> }.into_any(),
                    "Align Horizontal Distribute Start",
                ),
                (
                    view! { <AlignHorizontalJustifyCenter/> }.into_any(),
                    "Align Horizontal Justify Center",
                ),
                (view! { <AlignHorizontalJustifyEnd/> }.into_any(), "Align Horizontal Justify End"),
                (
                    view! { <AlignHorizontalJustifyStart/> }.into_any(),
                    "Align Horizontal Justify Start",
                ),
                (
                    view! { <AlignHorizontalSpaceAround/> }.into_any(),
                    "Align Horizontal Space Around",
                ),
                (
                    view! { <AlignHorizontalSpaceBetween/> }.into_any(),
                    "Align Horizontal Space Between",
                ),
                (view! { <AlignJustify/> }.into_any(), "Align Justify"),
                (view! { <AlignLeft/> }.into_any(), "Align Left"),
                (view! { <AlignRight/> }.into_any(), "Align Right"),
                (view! { <AlignStartHorizontal/> }.into_any(), "Align Start Horizontal"),
                (view! { <AlignStartVertical/> }.into_any(), "Align Start Vertical"),
                (
                    view! { <AlignVerticalDistributeCenter/> }.into_any(),
                    "Align Vertical Distribute Center",
                ),
                (
                    view! { <AlignVerticalDistributeEnd/> }.into_any(),
                    "Align Vertical Distribute End",
                ),
                (
                    view! { <AlignVerticalDistributeStart/> }.into_any(),
                    "Align Vertical Distribute Start",
                ),
                (
                    view! { <AlignVerticalJustifyCenter/> }.into_any(),
                    "Align Vertical Justify Center",
                ),
                (view! { <AlignVerticalJustifyEnd/> }.into_any(), "Align Vertical Justify End"),
                (view! { <AlignVerticalJustifyStart/> }.into_any(), "Align Vertical Justify Start"),
                (view! { <AlignVerticalSpaceAround/> }.into_any(), "Align Vertical Space Around"),
                (view! { <AlignVerticalSpaceBetween/> }.into_any(), "Align Vertical Space Between"),
                (view! { <Ambulance/> }.into_any(), "Ambulance"),
                (view! { <Ampersand/> }.into_any(), "Ampersand"),
                (view! { <Ampersands/> }.into_any(), "Ampersands"),
                (view! { <Amphora/> }.into_any(), "Amphora"),
                (view! { <Anchor/> }.into_any(), "Anchor"),
                (view! { <Angry/> }.into_any(), "Angry"),
                (view! { <Annoyed/> }.into_any(), "Annoyed"),
                (view! { <Antenna/> }.into_any(), "Antenna"),
                (view! { <Anvil/> }.into_any(), "Anvil"),
                (view! { <Aperture/> }.into_any(), "Aperture"),
                (view! { <AppWindowMac/> }.into_any(), "App Window Mac"),
                (view! { <AppWindow/> }.into_any(), "App Window"),
                (view! { <Apple/> }.into_any(), "Apple"),
                (view! { <ArchiveRestore/> }.into_any(), "Archive Restore"),
                (view! { <ArchiveX/> }.into_any(), "Archive X"),
                (view! { <Archive/> }.into_any(), "Archive"),
                (view! { <Armchair/> }.into_any(), "Armchair"),
                (view! { <ArrowBigDownDash/> }.into_any(), "Arrow Big Down Dash"),
                (view! { <ArrowBigDown/> }.into_any(), "Arrow Big Down"),
                (view! { <ArrowBigLeftDash/> }.into_any(), "Arrow Big Left Dash"),
                (view! { <ArrowBigLeft/> }.into_any(), "Arrow Big Left"),
                (view! { <ArrowBigRightDash/> }.into_any(), "Arrow Big Right Dash"),
                (view! { <ArrowBigRight/> }.into_any(), "Arrow Big Right"),
                (view! { <ArrowBigUpDash/> }.into_any(), "Arrow Big Up Dash"),
                (view! { <ArrowBigUp/> }.into_any(), "Arrow Big Up"),
                (view! { <ArrowDown01/> }.into_any(), "Arrow Down 01"),
                (view! { <ArrowDown10/> }.into_any(), "Arrow Down 10"),
                (view! { <ArrowDownAZ/> }.into_any(), "Arrow Down Az"),
                (view! { <ArrowDownFromLine/> }.into_any(), "Arrow Down From Line"),
                (view! { <ArrowDownLeft/> }.into_any(), "Arrow Down Left"),
                (view! { <ArrowDownNarrowWide/> }.into_any(), "Arrow Down Narrow Wide"),
                (view! { <ArrowDownRight/> }.into_any(), "Arrow Down Right"),
                (view! { <ArrowDownToDot/> }.into_any(), "Arrow Down To Dot"),
                (view! { <ArrowDownToLine/> }.into_any(), "Arrow Down To Line"),
                (view! { <ArrowDownUp/> }.into_any(), "Arrow Down Up"),
                (view! { <ArrowDownWideNarrow/> }.into_any(), "Arrow Down Wide Narrow"),
                (view! { <ArrowDownZA/> }.into_any(), "Arrow Down Za"),
                (view! { <ArrowDown/> }.into_any(), "Arrow Down"),
                (view! { <ArrowLeftFromLine/> }.into_any(), "Arrow Left From Line"),
                (view! { <ArrowLeftRight/> }.into_any(), "Arrow Left Right"),
                (view! { <ArrowLeftToLine/> }.into_any(), "Arrow Left To Line"),
                (view! { <ArrowLeft/> }.into_any(), "Arrow Left"),
                (view! { <ArrowRightFromLine/> }.into_any(), "Arrow Right From Line"),
                (view! { <ArrowRightLeft/> }.into_any(), "Arrow Right Left"),
                (view! { <ArrowRightToLine/> }.into_any(), "Arrow Right To Line"),
                (view! { <ArrowRight/> }.into_any(), "Arrow Right"),
                (view! { <ArrowUp01/> }.into_any(), "Arrow Up 01"),
                (view! { <ArrowUp10/> }.into_any(), "Arrow Up 10"),
                (view! { <ArrowUpAZ/> }.into_any(), "Arrow Up Az"),
                (view! { <ArrowUpDown/> }.into_any(), "Arrow Up Down"),
                (view! { <ArrowUpFromDot/> }.into_any(), "Arrow Up From Dot"),
                (view! { <ArrowUpFromLine/> }.into_any(), "Arrow Up From Line"),
                (view! { <ArrowUpLeft/> }.into_any(), "Arrow Up Left"),
                (view! { <ArrowUpNarrowWide/> }.into_any(), "Arrow Up Narrow Wide"),
                (view! { <ArrowUpRight/> }.into_any(), "Arrow Up Right"),
                (view! { <ArrowUpToLine/> }.into_any(), "Arrow Up To Line"),
                (view! { <ArrowUpWideNarrow/> }.into_any(), "Arrow Up Wide Narrow"),
                (view! { <ArrowUpZA/> }.into_any(), "Arrow Up Za"),
                (view! { <ArrowUp/> }.into_any(), "Arrow Up"),
                (view! { <ArrowsUpFromLine/> }.into_any(), "Arrows Up From Line"),
                (view! { <Asterisk/> }.into_any(), "Asterisk"),
                (view! { <AtSign/> }.into_any(), "At Sign"),
                (view! { <Atom/> }.into_any(), "Atom"),
                (view! { <AudioLines/> }.into_any(), "Audio Lines"),
                (view! { <AudioWaveform/> }.into_any(), "Audio Waveform"),
                (view! { <Award/> }.into_any(), "Award"),
                (view! { <Axe/> }.into_any(), "Axe"),
                (view! { <Axis3D/> }.into_any(), "Axis 3 D"),
            ]
            key=|icon| icon.1
            children=move |(icon, name)| {
                view! {
                    <div class="flex flex-wrap items-center gap-4 text-sm">
                        {icon} <span>{name}</span>
                    </div>
                }
            }
        />
    }
}
#[component]
pub fn IconsB() -> impl IntoView {
    view! {
        <For
            each=move || [
                (view! { <Baby/> }.into_any(), "Baby"),
                (view! { <Backpack/> }.into_any(), "Backpack"),
                (view! { <BadgeAlert/> }.into_any(), "Badge Alert"),
                (view! { <BadgeCent/> }.into_any(), "Badge Cent"),
                (view! { <BadgeCheck/> }.into_any(), "Badge Check"),
                (view! { <BadgeDollarSign/> }.into_any(), "Badge Dollar Sign"),
                (view! { <BadgeEuro/> }.into_any(), "Badge Euro"),
                (view! { <BadgeHelp/> }.into_any(), "Badge Help"),
                (view! { <BadgeIndianRupee/> }.into_any(), "Badge Indian Rupee"),
                (view! { <BadgeInfo/> }.into_any(), "Badge Info"),
                (view! { <BadgeJapaneseYen/> }.into_any(), "Badge Japanese Yen"),
                (view! { <BadgeMinus/> }.into_any(), "Badge Minus"),
                (view! { <BadgePercent/> }.into_any(), "Badge Percent"),
                (view! { <BadgePlus/> }.into_any(), "Badge Plus"),
                (view! { <BadgePoundSterling/> }.into_any(), "Badge Pound Sterling"),
                (view! { <BadgeRussianRuble/> }.into_any(), "Badge Russian Ruble"),
                (view! { <BadgeSwissFranc/> }.into_any(), "Badge Swiss Franc"),
                (view! { <BadgeX/> }.into_any(), "Badge X"),
                (view! { <Badge/> }.into_any(), "Badge"),
                (view! { <BaggageClaim/> }.into_any(), "Baggage Claim"),
                (view! { <Ban/> }.into_any(), "Ban"),
                (view! { <Banana/> }.into_any(), "Banana"),
                (view! { <Bandage/> }.into_any(), "Bandage"),
                (view! { <Banknote/> }.into_any(), "Banknote"),
                (view! { <Barcode/> }.into_any(), "Barcode"),
                (view! { <Baseline/> }.into_any(), "Baseline"),
                (view! { <Bath/> }.into_any(), "Bath"),
                (view! { <BatteryCharging/> }.into_any(), "Battery Charging"),
                (view! { <BatteryFull/> }.into_any(), "Battery Full"),
                (view! { <BatteryLow/> }.into_any(), "Battery Low"),
                (view! { <BatteryMedium/> }.into_any(), "Battery Medium"),
                (view! { <BatteryWarning/> }.into_any(), "Battery Warning"),
                (view! { <Battery/> }.into_any(), "Battery"),
                (view! { <Beaker/> }.into_any(), "Beaker"),
                (view! { <BeanOff/> }.into_any(), "Bean Off"),
                (view! { <Bean/> }.into_any(), "Bean"),
                (view! { <BedDouble/> }.into_any(), "Bed Double"),
                (view! { <BedSingle/> }.into_any(), "Bed Single"),
                (view! { <Bed/> }.into_any(), "Bed"),
                (view! { <Beef/> }.into_any(), "Beef"),
                (view! { <BeerOff/> }.into_any(), "Beer Off"),
                (view! { <Beer/> }.into_any(), "Beer"),
                (view! { <BellDot/> }.into_any(), "Bell Dot"),
                (view! { <BellElectric/> }.into_any(), "Bell Electric"),
                (view! { <BellMinus/> }.into_any(), "Bell Minus"),
                (view! { <BellOff/> }.into_any(), "Bell Off"),
                (view! { <BellPlus/> }.into_any(), "Bell Plus"),
                (view! { <BellRing/> }.into_any(), "Bell Ring"),
                (view! { <Bell/> }.into_any(), "Bell"),
                (view! { <BetweenHorizontalEnd/> }.into_any(), "Between Horizontal End"),
                (view! { <BetweenHorizontalStart/> }.into_any(), "Between Horizontal Start"),
                (view! { <BetweenVerticalEnd/> }.into_any(), "Between Vertical End"),
                (view! { <BetweenVerticalStart/> }.into_any(), "Between Vertical Start"),
                (view! { <BicepsFlexed/> }.into_any(), "Biceps Flexed"),
                (view! { <Bike/> }.into_any(), "Bike"),
                (view! { <Binary/> }.into_any(), "Binary"),
                (view! { <Binoculars/> }.into_any(), "Binoculars"),
                (view! { <Biohazard/> }.into_any(), "Biohazard"),
                (view! { <Bird/> }.into_any(), "Bird"),
                (view! { <Bitcoin/> }.into_any(), "Bitcoin"),
                (view! { <Blend/> }.into_any(), "Blend"),
                (view! { <Blinds/> }.into_any(), "Blinds"),
                (view! { <Blocks/> }.into_any(), "Blocks"),
                (view! { <BluetoothConnected/> }.into_any(), "Bluetooth Connected"),
                (view! { <BluetoothOff/> }.into_any(), "Bluetooth Off"),
                (view! { <BluetoothSearching/> }.into_any(), "Bluetooth Searching"),
                (view! { <Bluetooth/> }.into_any(), "Bluetooth"),
                (view! { <Bold/> }.into_any(), "Bold"),
                (view! { <Bolt/> }.into_any(), "Bolt"),
                (view! { <Bomb/> }.into_any(), "Bomb"),
                (view! { <Bone/> }.into_any(), "Bone"),
                (view! { <BookA/> }.into_any(), "Book A"),
                (view! { <BookAudio/> }.into_any(), "Book Audio"),
                (view! { <BookCheck/> }.into_any(), "Book Check"),
                (view! { <BookCopy/> }.into_any(), "Book Copy"),
                (view! { <BookDashed/> }.into_any(), "Book Dashed"),
                (view! { <BookDown/> }.into_any(), "Book Down"),
                (view! { <BookHeadphones/> }.into_any(), "Book Headphones"),
                (view! { <BookHeart/> }.into_any(), "Book Heart"),
                (view! { <BookImage/> }.into_any(), "Book Image"),
                (view! { <BookKey/> }.into_any(), "Book Key"),
                (view! { <BookLock/> }.into_any(), "Book Lock"),
                (view! { <BookMarked/> }.into_any(), "Book Marked"),
                (view! { <BookMinus/> }.into_any(), "Book Minus"),
                (view! { <BookOpenCheck/> }.into_any(), "Book Open Check"),
                (view! { <BookOpenText/> }.into_any(), "Book Open Text"),
                (view! { <BookOpen/> }.into_any(), "Book Open"),
                (view! { <BookPlus/> }.into_any(), "Book Plus"),
                (view! { <BookText/> }.into_any(), "Book Text"),
                (view! { <BookType/> }.into_any(), "Book Type"),
                (view! { <BookUp2/> }.into_any(), "Book Up 2"),
                (view! { <BookUp/> }.into_any(), "Book Up"),
                (view! { <BookUser/> }.into_any(), "Book User"),
                (view! { <BookX/> }.into_any(), "Book X"),
                (view! { <Book/> }.into_any(), "Book"),
                (view! { <BookmarkCheck/> }.into_any(), "Bookmark Check"),
                (view! { <BookmarkMinus/> }.into_any(), "Bookmark Minus"),
                (view! { <BookmarkPlus/> }.into_any(), "Bookmark Plus"),
                (view! { <BookmarkX/> }.into_any(), "Bookmark X"),
                (view! { <Bookmark/> }.into_any(), "Bookmark"),
                (view! { <BoomBox/> }.into_any(), "Boom Box"),
                (view! { <BotMessageSquare/> }.into_any(), "Bot Message Square"),
                (view! { <BotOff/> }.into_any(), "Bot Off"),
                (view! { <Bot/> }.into_any(), "Bot"),
                (view! { <Box/> }.into_any(), "Box"),
                (view! { <Boxes/> }.into_any(), "Boxes"),
                (view! { <Braces/> }.into_any(), "Braces"),
                (view! { <Brackets/> }.into_any(), "Brackets"),
                (view! { <BrainCircuit/> }.into_any(), "Brain Circuit"),
                (view! { <BrainCog/> }.into_any(), "Brain Cog"),
                (view! { <Brain/> }.into_any(), "Brain"),
                (view! { <BrickWall/> }.into_any(), "Brick Wall"),
                (view! { <BriefcaseBusiness/> }.into_any(), "Briefcase Business"),
                (view! { <BriefcaseConveyorBelt/> }.into_any(), "Briefcase Conveyor Belt"),
                (view! { <BriefcaseMedical/> }.into_any(), "Briefcase Medical"),
                (view! { <Briefcase/> }.into_any(), "Briefcase"),
                (view! { <BringToFront/> }.into_any(), "Bring To Front"),
                (view! { <Brush/> }.into_any(), "Brush"),
                (view! { <BugOff/> }.into_any(), "Bug Off"),
                (view! { <BugPlay/> }.into_any(), "Bug Play"),
                (view! { <Bug/> }.into_any(), "Bug"),
                (view! { <Building2/> }.into_any(), "Building 2"),
                (view! { <Building/> }.into_any(), "Building"),
                (view! { <BusFront/> }.into_any(), "Bus Front"),
                (view! { <Bus/> }.into_any(), "Bus"),
            ]
            key=|icon| icon.1
            children=move |(icon, name)| {
                view! {
                    <div class="flex flex-wrap items-center gap-4 text-sm">
                        {icon} <span>{name}</span>
                    </div>
                }
            }
        />
    }
}
#[component]
pub fn IconsC() -> impl IntoView {
    view! {
        <For
            each=move || [
                (view! { <CableCar/> }.into_any(), "Cable Car"),
                (view! { <Cable/> }.into_any(), "Cable"),
                (view! { <CakeSlice/> }.into_any(), "Cake Slice"),
                (view! { <Cake/> }.into_any(), "Cake"),
                (view! { <Calculator/> }.into_any(), "Calculator"),
                (view! { <Calendar1/> }.into_any(), "Calendar 1"),
                (view! { <CalendarArrowDown/> }.into_any(), "Calendar Arrow Down"),
                (view! { <CalendarArrowUp/> }.into_any(), "Calendar Arrow Up"),
                (view! { <CalendarCheck2/> }.into_any(), "Calendar Check 2"),
                (view! { <CalendarCheck/> }.into_any(), "Calendar Check"),
                (view! { <CalendarClock/> }.into_any(), "Calendar Clock"),
                (view! { <CalendarCog/> }.into_any(), "Calendar Cog"),
                (view! { <CalendarDays/> }.into_any(), "Calendar Days"),
                (view! { <CalendarFold/> }.into_any(), "Calendar Fold"),
                (view! { <CalendarHeart/> }.into_any(), "Calendar Heart"),
                (view! { <CalendarMinus2/> }.into_any(), "Calendar Minus 2"),
                (view! { <CalendarMinus/> }.into_any(), "Calendar Minus"),
                (view! { <CalendarOff/> }.into_any(), "Calendar Off"),
                (view! { <CalendarPlus2/> }.into_any(), "Calendar Plus 2"),
                (view! { <CalendarPlus/> }.into_any(), "Calendar Plus"),
                (view! { <CalendarRange/> }.into_any(), "Calendar Range"),
                (view! { <CalendarSearch/> }.into_any(), "Calendar Search"),
                (view! { <CalendarX2/> }.into_any(), "Calendar X 2"),
                (view! { <CalendarX/> }.into_any(), "Calendar X"),
                (view! { <Calendar/> }.into_any(), "Calendar"),
                (view! { <CameraOff/> }.into_any(), "Camera Off"),
                (view! { <Camera/> }.into_any(), "Camera"),
                (view! { <CandyCane/> }.into_any(), "Candy Cane"),
                (view! { <CandyOff/> }.into_any(), "Candy Off"),
                (view! { <Candy/> }.into_any(), "Candy"),
                (view! { <Cannabis/> }.into_any(), "Cannabis"),
                (view! { <CaptionsOff/> }.into_any(), "Captions Off"),
                (view! { <Captions/> }.into_any(), "Captions"),
                (view! { <CarFront/> }.into_any(), "Car Front"),
                (view! { <CarTaxiFront/> }.into_any(), "Car Taxi Front"),
                (view! { <Car/> }.into_any(), "Car"),
                (view! { <Caravan/> }.into_any(), "Caravan"),
                (view! { <Carrot/> }.into_any(), "Carrot"),
                (view! { <CaseLower/> }.into_any(), "Case Lower"),
                (view! { <CaseSensitive/> }.into_any(), "Case Sensitive"),
                (view! { <CaseUpper/> }.into_any(), "Case Upper"),
                (view! { <CassetteTape/> }.into_any(), "Cassette Tape"),
                (view! { <Cast/> }.into_any(), "Cast"),
                (view! { <Castle/> }.into_any(), "Castle"),
                (view! { <Cat/> }.into_any(), "Cat"),
                (view! { <Cctv/> }.into_any(), "Cctv"),
                (view! { <ChartArea/> }.into_any(), "Chart Area"),
                (view! { <ChartBarBig/> }.into_any(), "Chart Bar Big"),
                (view! { <ChartBarDecreasing/> }.into_any(), "Chart Bar Decreasing"),
                (view! { <ChartBarIncreasing/> }.into_any(), "Chart Bar Increasing"),
                (view! { <ChartBarStacked/> }.into_any(), "Chart Bar Stacked"),
                (view! { <ChartBar/> }.into_any(), "Chart Bar"),
                (view! { <ChartCandlestick/> }.into_any(), "Chart Candlestick"),
                (view! { <ChartColumnBig/> }.into_any(), "Chart Column Big"),
                (view! { <ChartColumnDecreasing/> }.into_any(), "Chart Column Decreasing"),
                (view! { <ChartColumnIncreasing/> }.into_any(), "Chart Column Increasing"),
                (view! { <ChartColumnStacked/> }.into_any(), "Chart Column Stacked"),
                (view! { <ChartColumn/> }.into_any(), "Chart Column"),
                (view! { <ChartGantt/> }.into_any(), "Chart Gantt"),
                (view! { <ChartLine/> }.into_any(), "Chart Line"),
                (view! { <ChartNetwork/> }.into_any(), "Chart Network"),
                (
                    view! { <ChartNoAxesColumnDecreasing/> }.into_any(),
                    "Chart No Axes Column Decreasing",
                ),
                (
                    view! { <ChartNoAxesColumnIncreasing/> }.into_any(),
                    "Chart No Axes Column Increasing",
                ),
                (view! { <ChartNoAxesColumn/> }.into_any(), "Chart No Axes Column"),
                (view! { <ChartNoAxesCombined/> }.into_any(), "Chart No Axes Combined"),
                (view! { <ChartNoAxesGantt/> }.into_any(), "Chart No Axes Gantt"),
                (view! { <ChartPie/> }.into_any(), "Chart Pie"),
                (view! { <ChartScatter/> }.into_any(), "Chart Scatter"),
                (view! { <ChartSpline/> }.into_any(), "Chart Spline"),
                (view! { <CheckCheck/> }.into_any(), "Check Check"),
                (view! { <Check/> }.into_any(), "Check"),
                (view! { <ChefHat/> }.into_any(), "Chef Hat"),
                (view! { <Cherry/> }.into_any(), "Cherry"),
                (view! { <ChevronDown/> }.into_any(), "Chevron Down"),
                (view! { <ChevronFirst/> }.into_any(), "Chevron First"),
                (view! { <ChevronLast/> }.into_any(), "Chevron Last"),
                (view! { <ChevronLeft/> }.into_any(), "Chevron Left"),
                (view! { <ChevronRight/> }.into_any(), "Chevron Right"),
                (view! { <ChevronUp/> }.into_any(), "Chevron Up"),
                (view! { <ChevronsDownUp/> }.into_any(), "Chevrons Down Up"),
                (view! { <ChevronsDown/> }.into_any(), "Chevrons Down"),
                (view! { <ChevronsLeftRightEllipsis/> }.into_any(), "Chevrons Left Right Ellipsis"),
                (view! { <ChevronsLeftRight/> }.into_any(), "Chevrons Left Right"),
                (view! { <ChevronsLeft/> }.into_any(), "Chevrons Left"),
                (view! { <ChevronsRightLeft/> }.into_any(), "Chevrons Right Left"),
                (view! { <ChevronsRight/> }.into_any(), "Chevrons Right"),
                (view! { <ChevronsUpDown/> }.into_any(), "Chevrons Up Down"),
                (view! { <ChevronsUp/> }.into_any(), "Chevrons Up"),
                (view! { <Chrome/> }.into_any(), "Chrome"),
                (view! { <Church/> }.into_any(), "Church"),
                (view! { <CigaretteOff/> }.into_any(), "Cigarette Off"),
                (view! { <Cigarette/> }.into_any(), "Cigarette"),
                (view! { <CircleAlert/> }.into_any(), "Circle Alert"),
                (view! { <CircleArrowDown/> }.into_any(), "Circle Arrow Down"),
                (view! { <CircleArrowLeft/> }.into_any(), "Circle Arrow Left"),
                (view! { <CircleArrowOutDownLeft/> }.into_any(), "Circle Arrow Out Down Left"),
                (view! { <CircleArrowOutDownRight/> }.into_any(), "Circle Arrow Out Down Right"),
                (view! { <CircleArrowOutUpLeft/> }.into_any(), "Circle Arrow Out Up Left"),
                (view! { <CircleArrowOutUpRight/> }.into_any(), "Circle Arrow Out Up Right"),
                (view! { <CircleArrowRight/> }.into_any(), "Circle Arrow Right"),
                (view! { <CircleArrowUp/> }.into_any(), "Circle Arrow Up"),
                (view! { <CircleCheckBig/> }.into_any(), "Circle Check Big"),
                (view! { <CircleCheck/> }.into_any(), "Circle Check"),
                (view! { <CircleChevronDown/> }.into_any(), "Circle Chevron Down"),
                (view! { <CircleChevronLeft/> }.into_any(), "Circle Chevron Left"),
                (view! { <CircleChevronRight/> }.into_any(), "Circle Chevron Right"),
                (view! { <CircleChevronUp/> }.into_any(), "Circle Chevron Up"),
                (view! { <CircleDashed/> }.into_any(), "Circle Dashed"),
                (view! { <CircleDivide/> }.into_any(), "Circle Divide"),
                (view! { <CircleDollarSign/> }.into_any(), "Circle Dollar Sign"),
                (view! { <CircleDotDashed/> }.into_any(), "Circle Dot Dashed"),
                (view! { <CircleDot/> }.into_any(), "Circle Dot"),
                (view! { <CircleEllipsis/> }.into_any(), "Circle Ellipsis"),
                (view! { <CircleEqual/> }.into_any(), "Circle Equal"),
                (view! { <CircleFadingArrowUp/> }.into_any(), "Circle Fading Arrow Up"),
                (view! { <CircleFadingPlus/> }.into_any(), "Circle Fading Plus"),
                (view! { <CircleGauge/> }.into_any(), "Circle Gauge"),
                (view! { <CircleHelp/> }.into_any(), "Circle Help"),
                (view! { <CircleMinus/> }.into_any(), "Circle Minus"),
                (view! { <CircleOff/> }.into_any(), "Circle Off"),
                (view! { <CircleParkingOff/> }.into_any(), "Circle Parking Off"),
                (view! { <CircleParking/> }.into_any(), "Circle Parking"),
                (view! { <CirclePause/> }.into_any(), "Circle Pause"),
                (view! { <CirclePercent/> }.into_any(), "Circle Percent"),
                (view! { <CirclePlay/> }.into_any(), "Circle Play"),
                (view! { <CirclePlus/> }.into_any(), "Circle Plus"),
                (view! { <CirclePower/> }.into_any(), "Circle Power"),
                (view! { <CircleSlash2/> }.into_any(), "Circle Slash 2"),
                (view! { <CircleSlash/> }.into_any(), "Circle Slash"),
                (view! { <CircleStop/> }.into_any(), "Circle Stop"),
                (view! { <CircleUserRound/> }.into_any(), "Circle User Round"),
                (view! { <CircleUser/> }.into_any(), "Circle User"),
                (view! { <CircleX/> }.into_any(), "Circle X"),
                (view! { <Circle/> }.into_any(), "Circle"),
                (view! { <CircuitBoard/> }.into_any(), "Circuit Board"),
                (view! { <Citrus/> }.into_any(), "Citrus"),
                (view! { <Clapperboard/> }.into_any(), "Clapperboard"),
                (view! { <ClipboardCheck/> }.into_any(), "Clipboard Check"),
                (view! { <ClipboardCopy/> }.into_any(), "Clipboard Copy"),
                (view! { <ClipboardList/> }.into_any(), "Clipboard List"),
                (view! { <ClipboardMinus/> }.into_any(), "Clipboard Minus"),
                (view! { <ClipboardPaste/> }.into_any(), "Clipboard Paste"),
                (view! { <ClipboardPenLine/> }.into_any(), "Clipboard Pen Line"),
                (view! { <ClipboardPen/> }.into_any(), "Clipboard Pen"),
                (view! { <ClipboardPlus/> }.into_any(), "Clipboard Plus"),
                (view! { <ClipboardType/> }.into_any(), "Clipboard Type"),
                (view! { <ClipboardX/> }.into_any(), "Clipboard X"),
                (view! { <Clipboard/> }.into_any(), "Clipboard"),
                (view! { <Clock1/> }.into_any(), "Clock 1"),
                (view! { <Clock10/> }.into_any(), "Clock 10"),
                (view! { <Clock11/> }.into_any(), "Clock 11"),
                (view! { <Clock12/> }.into_any(), "Clock 12"),
                (view! { <Clock2/> }.into_any(), "Clock 2"),
                (view! { <Clock3/> }.into_any(), "Clock 3"),
                (view! { <Clock4/> }.into_any(), "Clock 4"),
                (view! { <Clock5/> }.into_any(), "Clock 5"),
                (view! { <Clock6/> }.into_any(), "Clock 6"),
                (view! { <Clock7/> }.into_any(), "Clock 7"),
                (view! { <Clock8/> }.into_any(), "Clock 8"),
                (view! { <Clock9/> }.into_any(), "Clock 9"),
                (view! { <ClockAlert/> }.into_any(), "Clock Alert"),
                (view! { <ClockArrowDown/> }.into_any(), "Clock Arrow Down"),
                (view! { <ClockArrowUp/> }.into_any(), "Clock Arrow Up"),
                (view! { <Clock/> }.into_any(), "Clock"),
                (view! { <CloudAlert/> }.into_any(), "Cloud Alert"),
                (view! { <CloudCog/> }.into_any(), "Cloud Cog"),
                (view! { <CloudDownload/> }.into_any(), "Cloud Download"),
                (view! { <CloudDrizzle/> }.into_any(), "Cloud Drizzle"),
                (view! { <CloudFog/> }.into_any(), "Cloud Fog"),
                (view! { <CloudHail/> }.into_any(), "Cloud Hail"),
                (view! { <CloudLightning/> }.into_any(), "Cloud Lightning"),
                (view! { <CloudMoonRain/> }.into_any(), "Cloud Moon Rain"),
                (view! { <CloudMoon/> }.into_any(), "Cloud Moon"),
                (view! { <CloudOff/> }.into_any(), "Cloud Off"),
                (view! { <CloudRainWind/> }.into_any(), "Cloud Rain Wind"),
                (view! { <CloudRain/> }.into_any(), "Cloud Rain"),
                (view! { <CloudSnow/> }.into_any(), "Cloud Snow"),
                (view! { <CloudSunRain/> }.into_any(), "Cloud Sun Rain"),
                (view! { <CloudSun/> }.into_any(), "Cloud Sun"),
                (view! { <CloudUpload/> }.into_any(), "Cloud Upload"),
                (view! { <Cloud/> }.into_any(), "Cloud"),
                (view! { <Cloudy/> }.into_any(), "Cloudy"),
                (view! { <Clover/> }.into_any(), "Clover"),
                (view! { <Club/> }.into_any(), "Club"),
                (view! { <CodeXml/> }.into_any(), "Code Xml"),
                (view! { <Code/> }.into_any(), "Code"),
                (view! { <Codepen/> }.into_any(), "Codepen"),
                (view! { <Codesandbox/> }.into_any(), "Codesandbox"),
                (view! { <Coffee/> }.into_any(), "Coffee"),
                (view! { <Cog/> }.into_any(), "Cog"),
                (view! { <Coins/> }.into_any(), "Coins"),
                (view! { <Columns2/> }.into_any(), "Columns 2"),
                (view! { <Columns3/> }.into_any(), "Columns 3"),
                (view! { <Columns4/> }.into_any(), "Columns 4"),
                (view! { <Combine/> }.into_any(), "Combine"),
                (view! { <Command/> }.into_any(), "Command"),
                (view! { <Compass/> }.into_any(), "Compass"),
                (view! { <Component/> }.into_any(), "Component"),
                (view! { <Computer/> }.into_any(), "Computer"),
                (view! { <ConciergeBell/> }.into_any(), "Concierge Bell"),
                (view! { <Cone/> }.into_any(), "Cone"),
                (view! { <Construction/> }.into_any(), "Construction"),
                (view! { <ContactRound/> }.into_any(), "Contact Round"),
                (view! { <Contact/> }.into_any(), "Contact"),
                (view! { <Container/> }.into_any(), "Container"),
                (view! { <Contrast/> }.into_any(), "Contrast"),
                (view! { <Cookie/> }.into_any(), "Cookie"),
                (view! { <CookingPot/> }.into_any(), "Cooking Pot"),
                (view! { <CopyCheck/> }.into_any(), "Copy Check"),
                (view! { <CopyMinus/> }.into_any(), "Copy Minus"),
                (view! { <CopyPlus/> }.into_any(), "Copy Plus"),
                (view! { <CopySlash/> }.into_any(), "Copy Slash"),
                (view! { <CopyX/> }.into_any(), "Copy X"),
                (view! { <Copy/> }.into_any(), "Copy"),
                (view! { <Copyleft/> }.into_any(), "Copyleft"),
                (view! { <Copyright/> }.into_any(), "Copyright"),
                (view! { <CornerDownLeft/> }.into_any(), "Corner Down Left"),
                (view! { <CornerDownRight/> }.into_any(), "Corner Down Right"),
                (view! { <CornerLeftDown/> }.into_any(), "Corner Left Down"),
                (view! { <CornerLeftUp/> }.into_any(), "Corner Left Up"),
                (view! { <CornerRightDown/> }.into_any(), "Corner Right Down"),
                (view! { <CornerRightUp/> }.into_any(), "Corner Right Up"),
                (view! { <CornerUpLeft/> }.into_any(), "Corner Up Left"),
                (view! { <CornerUpRight/> }.into_any(), "Corner Up Right"),
                (view! { <Cpu/> }.into_any(), "Cpu"),
                (view! { <CreativeCommons/> }.into_any(), "Creative Commons"),
                (view! { <CreditCard/> }.into_any(), "Credit Card"),
                (view! { <Croissant/> }.into_any(), "Croissant"),
                (view! { <Crop/> }.into_any(), "Crop"),
                (view! { <Cross/> }.into_any(), "Cross"),
                (view! { <Crosshair/> }.into_any(), "Crosshair"),
                (view! { <Crown/> }.into_any(), "Crown"),
                (view! { <Cuboid/> }.into_any(), "Cuboid"),
                (view! { <CupSoda/> }.into_any(), "Cup Soda"),
                (view! { <Currency/> }.into_any(), "Currency"),
                (view! { <Cylinder/> }.into_any(), "Cylinder"),
            ]
            key=|icon| icon.1
            children=move |(icon, name)| {
                view! {
                    <div class="flex flex-wrap items-center gap-4 text-sm">
                        {icon} <span>{name}</span>
                    </div>
                }
            }
        />
    }
}
#[component]
pub fn IconsD() -> impl IntoView {
    view! {
        <For
            each=move || [
                (view! { <Dam/> }.into_any(), "Dam"),
                (view! { <DatabaseBackup/> }.into_any(), "Database Backup"),
                (view! { <DatabaseZap/> }.into_any(), "Database Zap"),
                (view! { <Database/> }.into_any(), "Database"),
                (view! { <Delete/> }.into_any(), "Delete"),
                (view! { <Dessert/> }.into_any(), "Dessert"),
                (view! { <Diameter/> }.into_any(), "Diameter"),
                (view! { <DiamondMinus/> }.into_any(), "Diamond Minus"),
                (view! { <DiamondPercent/> }.into_any(), "Diamond Percent"),
                (view! { <DiamondPlus/> }.into_any(), "Diamond Plus"),
                (view! { <Diamond/> }.into_any(), "Diamond"),
                (view! { <Dice1/> }.into_any(), "Dice 1"),
                (view! { <Dice2/> }.into_any(), "Dice 2"),
                (view! { <Dice3/> }.into_any(), "Dice 3"),
                (view! { <Dice4/> }.into_any(), "Dice 4"),
                (view! { <Dice5/> }.into_any(), "Dice 5"),
                (view! { <Dice6/> }.into_any(), "Dice 6"),
                (view! { <Dices/> }.into_any(), "Dices"),
                (view! { <Diff/> }.into_any(), "Diff"),
                (view! { <Disc2/> }.into_any(), "Disc 2"),
                (view! { <Disc3/> }.into_any(), "Disc 3"),
                (view! { <DiscAlbum/> }.into_any(), "Disc Album"),
                (view! { <Disc/> }.into_any(), "Disc"),
                (view! { <Divide/> }.into_any(), "Divide"),
                (view! { <DnaOff/> }.into_any(), "Dna Off"),
                (view! { <Dna/> }.into_any(), "Dna"),
                (view! { <Dock/> }.into_any(), "Dock"),
                (view! { <Dog/> }.into_any(), "Dog"),
                (view! { <DollarSign/> }.into_any(), "Dollar Sign"),
                (view! { <Donut/> }.into_any(), "Donut"),
                (view! { <DoorClosed/> }.into_any(), "Door Closed"),
                (view! { <DoorOpen/> }.into_any(), "Door Open"),
                (view! { <Dot/> }.into_any(), "Dot"),
                (view! { <Download/> }.into_any(), "Download"),
                (view! { <DraftingCompass/> }.into_any(), "Drafting Compass"),
                (view! { <Drama/> }.into_any(), "Drama"),
                (view! { <Dribbble/> }.into_any(), "Dribbble"),
                (view! { <Drill/> }.into_any(), "Drill"),
                (view! { <Droplet/> }.into_any(), "Droplet"),
                (view! { <Droplets/> }.into_any(), "Droplets"),
                (view! { <Drum/> }.into_any(), "Drum"),
                (view! { <Drumstick/> }.into_any(), "Drumstick"),
                (view! { <Dumbbell/> }.into_any(), "Dumbbell"),
            ]
            key=|icon| icon.1
            children=move |(icon, name)| {
                view! {
                    <div class="flex flex-wrap items-center gap-4 text-sm">
                        {icon} <span>{name}</span>
                    </div>
                }
            }
        />
    }
}
#[component]
pub fn IconsE() -> impl IntoView {
    view! {
        <For
            each=move || [
                (view! { <EarOff/> }.into_any(), "Ear Off"),
                (view! { <Ear/> }.into_any(), "Ear"),
                (view! { <EarthLock/> }.into_any(), "Earth Lock"),
                (view! { <Earth/> }.into_any(), "Earth"),
                (view! { <Eclipse/> }.into_any(), "Eclipse"),
                (view! { <EggFried/> }.into_any(), "Egg Fried"),
                (view! { <EggOff/> }.into_any(), "Egg Off"),
                (view! { <Egg/> }.into_any(), "Egg"),
                (view! { <EllipsisVertical/> }.into_any(), "Ellipsis Vertical"),
                (view! { <Ellipsis/> }.into_any(), "Ellipsis"),
                (view! { <EqualApproximately/> }.into_any(), "Equal Approximately"),
                (view! { <EqualNot/> }.into_any(), "Equal Not"),
                (view! { <Equal/> }.into_any(), "Equal"),
                (view! { <Eraser/> }.into_any(), "Eraser"),
                (view! { <EthernetPort/> }.into_any(), "Ethernet Port"),
                (view! { <Euro/> }.into_any(), "Euro"),
                (view! { <Expand/> }.into_any(), "Expand"),
                (view! { <ExternalLink/> }.into_any(), "External Link"),
                (view! { <EyeClosed/> }.into_any(), "Eye Closed"),
                (view! { <EyeOff/> }.into_any(), "Eye Off"),
                (view! { <Eye/> }.into_any(), "Eye"),
            ]
            key=|icon| icon.1
            children=move |(icon, name)| {
                view! {
                    <div class="flex flex-wrap items-center gap-4 text-sm">
                        {icon} <span>{name}</span>
                    </div>
                }
            }
        />
    }
}
#[component]
pub fn IconsF() -> impl IntoView {
    view! {
        <For
            each=move || [
                (view! { <Facebook/> }.into_any(), "Facebook"),
                (view! { <Factory/> }.into_any(), "Factory"),
                (view! { <Fan/> }.into_any(), "Fan"),
                (view! { <FastForward/> }.into_any(), "Fast Forward"),
                (view! { <Feather/> }.into_any(), "Feather"),
                (view! { <Fence/> }.into_any(), "Fence"),
                (view! { <FerrisWheel/> }.into_any(), "Ferris Wheel"),
                (view! { <Figma/> }.into_any(), "Figma"),
                (view! { <FileArchive/> }.into_any(), "File Archive"),
                (view! { <FileAudio2/> }.into_any(), "File Audio 2"),
                (view! { <FileAudio/> }.into_any(), "File Audio"),
                (view! { <FileAxis3D/> }.into_any(), "File Axis 3 D"),
                (view! { <FileBadge2/> }.into_any(), "File Badge 2"),
                (view! { <FileBadge/> }.into_any(), "File Badge"),
                (view! { <FileBox/> }.into_any(), "File Box"),
                (view! { <FileChartColumnIncreasing/> }.into_any(), "File Chart Column Increasing"),
                (view! { <FileChartColumn/> }.into_any(), "File Chart Column"),
                (view! { <FileChartLine/> }.into_any(), "File Chart Line"),
                (view! { <FileChartPie/> }.into_any(), "File Chart Pie"),
                (view! { <FileCheck2/> }.into_any(), "File Check 2"),
                (view! { <FileCheck/> }.into_any(), "File Check"),
                (view! { <FileClock/> }.into_any(), "File Clock"),
                (view! { <FileCode2/> }.into_any(), "File Code 2"),
                (view! { <FileCode/> }.into_any(), "File Code"),
                (view! { <FileCog/> }.into_any(), "File Cog"),
                (view! { <FileDiff/> }.into_any(), "File Diff"),
                (view! { <FileDigit/> }.into_any(), "File Digit"),
                (view! { <FileDown/> }.into_any(), "File Down"),
                (view! { <FileHeart/> }.into_any(), "File Heart"),
                (view! { <FileImage/> }.into_any(), "File Image"),
                (view! { <FileInput/> }.into_any(), "File Input"),
                (view! { <FileJson2/> }.into_any(), "File Json 2"),
                (view! { <FileJson/> }.into_any(), "File Json"),
                (view! { <FileKey2/> }.into_any(), "File Key 2"),
                (view! { <FileKey/> }.into_any(), "File Key"),
                (view! { <FileLock2/> }.into_any(), "File Lock 2"),
                (view! { <FileLock/> }.into_any(), "File Lock"),
                (view! { <FileMinus2/> }.into_any(), "File Minus 2"),
                (view! { <FileMinus/> }.into_any(), "File Minus"),
                (view! { <FileMusic/> }.into_any(), "File Music"),
                (view! { <FileOutput/> }.into_any(), "File Output"),
                (view! { <FilePenLine/> }.into_any(), "File Pen Line"),
                (view! { <FilePen/> }.into_any(), "File Pen"),
                (view! { <FilePlus2/> }.into_any(), "File Plus 2"),
                (view! { <FilePlus/> }.into_any(), "File Plus"),
                (view! { <FileQuestion/> }.into_any(), "File Question"),
                (view! { <FileScan/> }.into_any(), "File Scan"),
                (view! { <FileSearch2/> }.into_any(), "File Search 2"),
                (view! { <FileSearch/> }.into_any(), "File Search"),
                (view! { <FileSliders/> }.into_any(), "File Sliders"),
                (view! { <FileSpreadsheet/> }.into_any(), "File Spreadsheet"),
                (view! { <FileStack/> }.into_any(), "File Stack"),
                (view! { <FileSymlink/> }.into_any(), "File Symlink"),
                (view! { <FileTerminal/> }.into_any(), "File Terminal"),
                (view! { <FileText/> }.into_any(), "File Text"),
                (view! { <FileType2/> }.into_any(), "File Type 2"),
                (view! { <FileType/> }.into_any(), "File Type"),
                (view! { <FileUp/> }.into_any(), "File Up"),
                (view! { <FileUser/> }.into_any(), "File User"),
                (view! { <FileVideo2/> }.into_any(), "File Video 2"),
                (view! { <FileVideo/> }.into_any(), "File Video"),
                (view! { <FileVolume2/> }.into_any(), "File Volume 2"),
                (view! { <FileVolume/> }.into_any(), "File Volume"),
                (view! { <FileWarning/> }.into_any(), "File Warning"),
                (view! { <FileX2/> }.into_any(), "File X 2"),
                (view! { <FileX/> }.into_any(), "File X"),
                (view! { <File/> }.into_any(), "File"),
                (view! { <Files/> }.into_any(), "Files"),
                (view! { <Film/> }.into_any(), "Film"),
                (view! { <FilterX/> }.into_any(), "Filter X"),
                (view! { <Filter/> }.into_any(), "Filter"),
                (view! { <Fingerprint/> }.into_any(), "Fingerprint"),
                (view! { <FireExtinguisher/> }.into_any(), "Fire Extinguisher"),
                (view! { <FishOff/> }.into_any(), "Fish Off"),
                (view! { <FishSymbol/> }.into_any(), "Fish Symbol"),
                (view! { <Fish/> }.into_any(), "Fish"),
                (view! { <FlagOff/> }.into_any(), "Flag Off"),
                (view! { <FlagTriangleLeft/> }.into_any(), "Flag Triangle Left"),
                (view! { <FlagTriangleRight/> }.into_any(), "Flag Triangle Right"),
                (view! { <Flag/> }.into_any(), "Flag"),
                (view! { <FlameKindling/> }.into_any(), "Flame Kindling"),
                (view! { <Flame/> }.into_any(), "Flame"),
                (view! { <FlashlightOff/> }.into_any(), "Flashlight Off"),
                (view! { <Flashlight/> }.into_any(), "Flashlight"),
                (view! { <FlaskConicalOff/> }.into_any(), "Flask Conical Off"),
                (view! { <FlaskConical/> }.into_any(), "Flask Conical"),
                (view! { <FlaskRound/> }.into_any(), "Flask Round"),
                (view! { <FlipHorizontal2/> }.into_any(), "Flip Horizontal 2"),
                (view! { <FlipHorizontal/> }.into_any(), "Flip Horizontal"),
                (view! { <FlipVertical2/> }.into_any(), "Flip Vertical 2"),
                (view! { <FlipVertical/> }.into_any(), "Flip Vertical"),
                (view! { <Flower2/> }.into_any(), "Flower 2"),
                (view! { <Flower/> }.into_any(), "Flower"),
                (view! { <Focus/> }.into_any(), "Focus"),
                (view! { <FoldHorizontal/> }.into_any(), "Fold Horizontal"),
                (view! { <FoldVertical/> }.into_any(), "Fold Vertical"),
                (view! { <FolderArchive/> }.into_any(), "Folder Archive"),
                (view! { <FolderCheck/> }.into_any(), "Folder Check"),
                (view! { <FolderClock/> }.into_any(), "Folder Clock"),
                (view! { <FolderClosed/> }.into_any(), "Folder Closed"),
                (view! { <FolderCode/> }.into_any(), "Folder Code"),
                (view! { <FolderCog/> }.into_any(), "Folder Cog"),
                (view! { <FolderDot/> }.into_any(), "Folder Dot"),
                (view! { <FolderDown/> }.into_any(), "Folder Down"),
                (view! { <FolderGit2/> }.into_any(), "Folder Git 2"),
                (view! { <FolderGit/> }.into_any(), "Folder Git"),
                (view! { <FolderHeart/> }.into_any(), "Folder Heart"),
                (view! { <FolderInput/> }.into_any(), "Folder Input"),
                (view! { <FolderKanban/> }.into_any(), "Folder Kanban"),
                (view! { <FolderKey/> }.into_any(), "Folder Key"),
                (view! { <FolderLock/> }.into_any(), "Folder Lock"),
                (view! { <FolderMinus/> }.into_any(), "Folder Minus"),
                (view! { <FolderOpenDot/> }.into_any(), "Folder Open Dot"),
                (view! { <FolderOpen/> }.into_any(), "Folder Open"),
                (view! { <FolderOutput/> }.into_any(), "Folder Output"),
                (view! { <FolderPen/> }.into_any(), "Folder Pen"),
                (view! { <FolderPlus/> }.into_any(), "Folder Plus"),
                (view! { <FolderRoot/> }.into_any(), "Folder Root"),
                (view! { <FolderSearch2/> }.into_any(), "Folder Search 2"),
                (view! { <FolderSearch/> }.into_any(), "Folder Search"),
                (view! { <FolderSymlink/> }.into_any(), "Folder Symlink"),
                (view! { <FolderSync/> }.into_any(), "Folder Sync"),
                (view! { <FolderTree/> }.into_any(), "Folder Tree"),
                (view! { <FolderUp/> }.into_any(), "Folder Up"),
                (view! { <FolderX/> }.into_any(), "Folder X"),
                (view! { <Folder/> }.into_any(), "Folder"),
                (view! { <Folders/> }.into_any(), "Folders"),
                (view! { <Footprints/> }.into_any(), "Footprints"),
                (view! { <Forklift/> }.into_any(), "Forklift"),
                (view! { <Forward/> }.into_any(), "Forward"),
                (view! { <Frame/> }.into_any(), "Frame"),
                (view! { <Framer/> }.into_any(), "Framer"),
                (view! { <Frown/> }.into_any(), "Frown"),
                (view! { <Fuel/> }.into_any(), "Fuel"),
                (view! { <Fullscreen/> }.into_any(), "Fullscreen"),
            ]
            key=|icon| icon.1
            children=move |(icon, name)| {
                view! {
                    <div class="flex flex-wrap items-center gap-4 text-sm">
                        {icon} <span>{name}</span>
                    </div>
                }
            }
        />
    }
}
#[component]
pub fn IconsG() -> impl IntoView {
    view! {
        <For
            each=move || [
                (view! { <GalleryHorizontalEnd/> }.into_any(), "Gallery Horizontal End"),
                (view! { <GalleryHorizontal/> }.into_any(), "Gallery Horizontal"),
                (view! { <GalleryThumbnails/> }.into_any(), "Gallery Thumbnails"),
                (view! { <GalleryVerticalEnd/> }.into_any(), "Gallery Vertical End"),
                (view! { <GalleryVertical/> }.into_any(), "Gallery Vertical"),
                (view! { <Gamepad2/> }.into_any(), "Gamepad 2"),
                (view! { <Gamepad/> }.into_any(), "Gamepad"),
                (view! { <Gauge/> }.into_any(), "Gauge"),
                (view! { <Gavel/> }.into_any(), "Gavel"),
                (view! { <Gem/> }.into_any(), "Gem"),
                (view! { <Ghost/> }.into_any(), "Ghost"),
                (view! { <Gift/> }.into_any(), "Gift"),
                (view! { <GitBranchPlus/> }.into_any(), "Git Branch Plus"),
                (view! { <GitBranch/> }.into_any(), "Git Branch"),
                (view! { <GitCommitHorizontal/> }.into_any(), "Git Commit Horizontal"),
                (view! { <GitCommitVertical/> }.into_any(), "Git Commit Vertical"),
                (view! { <GitCompareArrows/> }.into_any(), "Git Compare Arrows"),
                (view! { <GitCompare/> }.into_any(), "Git Compare"),
                (view! { <GitFork/> }.into_any(), "Git Fork"),
                (view! { <GitGraph/> }.into_any(), "Git Graph"),
                (view! { <GitMerge/> }.into_any(), "Git Merge"),
                (view! { <GitPullRequestArrow/> }.into_any(), "Git Pull Request Arrow"),
                (view! { <GitPullRequestClosed/> }.into_any(), "Git Pull Request Closed"),
                (
                    view! { <GitPullRequestCreateArrow/> }.into_any(),
                    "Git Pull Request Create Arrow",
                ),
                (view! { <GitPullRequestCreate/> }.into_any(), "Git Pull Request Create"),
                (view! { <GitPullRequestDraft/> }.into_any(), "Git Pull Request Draft"),
                (view! { <GitPullRequest/> }.into_any(), "Git Pull Request"),
                (view! { <Github/> }.into_any(), "Github"),
                (view! { <Gitlab/> }.into_any(), "Gitlab"),
                (view! { <GlassWater/> }.into_any(), "Glass Water"),
                (view! { <Glasses/> }.into_any(), "Glasses"),
                (view! { <GlobeLock/> }.into_any(), "Globe Lock"),
                (view! { <Globe/> }.into_any(), "Globe"),
                (view! { <Goal/> }.into_any(), "Goal"),
                (view! { <Grab/> }.into_any(), "Grab"),
                (view! { <GraduationCap/> }.into_any(), "Graduation Cap"),
                (view! { <Grape/> }.into_any(), "Grape"),
                (view! { <Grid2X2Check/> }.into_any(), "Grid 2 X 2 Check"),
                (view! { <Grid2X2Plus/> }.into_any(), "Grid 2 X 2 Plus"),
                (view! { <Grid2X2X/> }.into_any(), "Grid 2 X 2 X"),
                (view! { <Grid2X2/> }.into_any(), "Grid 2 X 2"),
                (view! { <Grid3X3/> }.into_any(), "Grid 3 X 3"),
                (view! { <GripHorizontal/> }.into_any(), "Grip Horizontal"),
                (view! { <GripVertical/> }.into_any(), "Grip Vertical"),
                (view! { <Grip/> }.into_any(), "Grip"),
                (view! { <Group/> }.into_any(), "Group"),
                (view! { <Guitar/> }.into_any(), "Guitar"),
            ]
            key=|icon| icon.1
            children=move |(icon, name)| {
                view! {
                    <div class="flex flex-wrap items-center gap-4 text-sm">
                        {icon} <span>{name}</span>
                    </div>
                }
            }
        />
    }
}
#[component]
pub fn IconsH() -> impl IntoView {
    view! {
        <For
            each=move || [
                (view! { <Ham/> }.into_any(), "Ham"),
                (view! { <Hammer/> }.into_any(), "Hammer"),
                (view! { <HandCoins/> }.into_any(), "Hand Coins"),
                (view! { <HandHeart/> }.into_any(), "Hand Heart"),
                (view! { <HandHelping/> }.into_any(), "Hand Helping"),
                (view! { <HandMetal/> }.into_any(), "Hand Metal"),
                (view! { <HandPlatter/> }.into_any(), "Hand Platter"),
                (view! { <Hand/> }.into_any(), "Hand"),
                (view! { <Handshake/> }.into_any(), "Handshake"),
                (view! { <HardDriveDownload/> }.into_any(), "Hard Drive Download"),
                (view! { <HardDriveUpload/> }.into_any(), "Hard Drive Upload"),
                (view! { <HardDrive/> }.into_any(), "Hard Drive"),
                (view! { <HardHat/> }.into_any(), "Hard Hat"),
                (view! { <Hash/> }.into_any(), "Hash"),
                (view! { <Haze/> }.into_any(), "Haze"),
                (view! { <HdmiPort/> }.into_any(), "Hdmi Port"),
                (view! { <Heading1/> }.into_any(), "Heading 1"),
                (view! { <Heading2/> }.into_any(), "Heading 2"),
                (view! { <Heading3/> }.into_any(), "Heading 3"),
                (view! { <Heading4/> }.into_any(), "Heading 4"),
                (view! { <Heading5/> }.into_any(), "Heading 5"),
                (view! { <Heading6/> }.into_any(), "Heading 6"),
                (view! { <Heading/> }.into_any(), "Heading"),
                (view! { <HeadphoneOff/> }.into_any(), "Headphone Off"),
                (view! { <Headphones/> }.into_any(), "Headphones"),
                (view! { <Headset/> }.into_any(), "Headset"),
                (view! { <HeartCrack/> }.into_any(), "Heart Crack"),
                (view! { <HeartHandshake/> }.into_any(), "Heart Handshake"),
                (view! { <HeartOff/> }.into_any(), "Heart Off"),
                (view! { <HeartPulse/> }.into_any(), "Heart Pulse"),
                (view! { <Heart/> }.into_any(), "Heart"),
                (view! { <Heater/> }.into_any(), "Heater"),
                (view! { <Hexagon/> }.into_any(), "Hexagon"),
                (view! { <Highlighter/> }.into_any(), "Highlighter"),
                (view! { <History/> }.into_any(), "History"),
                (view! { <HopOff/> }.into_any(), "Hop Off"),
                (view! { <Hop/> }.into_any(), "Hop"),
                (view! { <Hospital/> }.into_any(), "Hospital"),
                (view! { <Hotel/> }.into_any(), "Hotel"),
                (view! { <Hourglass/> }.into_any(), "Hourglass"),
                (view! { <HousePlug/> }.into_any(), "House Plug"),
                (view! { <HousePlus/> }.into_any(), "House Plus"),
                (view! { <House/> }.into_any(), "House"),
            ]
            key=|icon| icon.1
            children=move |(icon, name)| {
                view! {
                    <div class="flex flex-wrap items-center gap-4 text-sm">
                        {icon} <span>{name}</span>
                    </div>
                }
            }
        />
    }
}
#[component]
pub fn IconsI() -> impl IntoView {
    view! {
        <For
            each=move || [
                (view! { <IceCreamBowl/> }.into_any(), "Ice Cream Bowl"),
                (view! { <IceCreamCone/> }.into_any(), "Ice Cream Cone"),
                (view! { <IdCard/> }.into_any(), "Id Card"),
                (view! { <ImageDown/> }.into_any(), "Image Down"),
                (view! { <ImageMinus/> }.into_any(), "Image Minus"),
                (view! { <ImageOff/> }.into_any(), "Image Off"),
                (view! { <ImagePlay/> }.into_any(), "Image Play"),
                (view! { <ImagePlus/> }.into_any(), "Image Plus"),
                (view! { <ImageUp/> }.into_any(), "Image Up"),
                (view! { <Image/> }.into_any(), "Image"),
                (view! { <Images/> }.into_any(), "Images"),
                (view! { <Import/> }.into_any(), "Import"),
                (view! { <Inbox/> }.into_any(), "Inbox"),
                (view! { <IndentDecrease/> }.into_any(), "Indent Decrease"),
                (view! { <IndentIncrease/> }.into_any(), "Indent Increase"),
                (view! { <IndianRupee/> }.into_any(), "Indian Rupee"),
                (view! { <Infinity/> }.into_any(), "Infinity"),
                (view! { <Info/> }.into_any(), "Info"),
                (view! { <InspectionPanel/> }.into_any(), "Inspection Panel"),
                (view! { <Instagram/> }.into_any(), "Instagram"),
                (view! { <Italic/> }.into_any(), "Italic"),
                (view! { <IterationCcw/> }.into_any(), "Iteration Ccw"),
                (view! { <IterationCw/> }.into_any(), "Iteration Cw"),
            ]
            key=|icon| icon.1
            children=move |(icon, name)| {
                view! {
                    <div class="flex flex-wrap items-center gap-4 text-sm">
                        {icon} <span>{name}</span>
                    </div>
                }
            }
        />
    }
}
#[component]
pub fn IconsJ() -> impl IntoView {
    view! {
        <For
            each=move || [
                (view! { <JapaneseYen/> }.into_any(), "Japanese Yen"),
                (view! { <Joystick/> }.into_any(), "Joystick"),
            ]
            key=|icon| icon.1
            children=move |(icon, name)| {
                view! {
                    <div class="flex flex-wrap items-center gap-4 text-sm">
                        {icon} <span>{name}</span>
                    </div>
                }
            }
        />
    }
}
#[component]
pub fn IconsK() -> impl IntoView {
    view! {
        <For
            each=move || [
                (view! { <Kanban/> }.into_any(), "Kanban"),
                (view! { <KeyRound/> }.into_any(), "Key Round"),
                (view! { <KeySquare/> }.into_any(), "Key Square"),
                (view! { <Key/> }.into_any(), "Key"),
                (view! { <KeyboardMusic/> }.into_any(), "Keyboard Music"),
                (view! { <KeyboardOff/> }.into_any(), "Keyboard Off"),
                (view! { <Keyboard/> }.into_any(), "Keyboard"),
            ]
            key=|icon| icon.1
            children=move |(icon, name)| {
                view! {
                    <div class="flex flex-wrap items-center gap-4 text-sm">
                        {icon} <span>{name}</span>
                    </div>
                }
            }
        />
    }
}
#[component]
pub fn IconsL() -> impl IntoView {
    view! {
        <For
            each=move || [
                (view! { <LampCeiling/> }.into_any(), "Lamp Ceiling"),
                (view! { <LampDesk/> }.into_any(), "Lamp Desk"),
                (view! { <LampFloor/> }.into_any(), "Lamp Floor"),
                (view! { <LampWallDown/> }.into_any(), "Lamp Wall Down"),
                (view! { <LampWallUp/> }.into_any(), "Lamp Wall Up"),
                (view! { <Lamp/> }.into_any(), "Lamp"),
                (view! { <LandPlot/> }.into_any(), "Land Plot"),
                (view! { <Landmark/> }.into_any(), "Landmark"),
                (view! { <Languages/> }.into_any(), "Languages"),
                (view! { <LaptopMinimalCheck/> }.into_any(), "Laptop Minimal Check"),
                (view! { <LaptopMinimal/> }.into_any(), "Laptop Minimal"),
                (view! { <Laptop/> }.into_any(), "Laptop"),
                (view! { <LassoSelect/> }.into_any(), "Lasso Select"),
                (view! { <Lasso/> }.into_any(), "Lasso"),
                (view! { <Laugh/> }.into_any(), "Laugh"),
                (view! { <Layers2/> }.into_any(), "Layers 2"),
                (view! { <Layers3/> }.into_any(), "Layers 3"),
                (view! { <Layers/> }.into_any(), "Layers"),
                (view! { <LayoutDashboard/> }.into_any(), "Layout Dashboard"),
                (view! { <LayoutGrid/> }.into_any(), "Layout Grid"),
                (view! { <LayoutList/> }.into_any(), "Layout List"),
                (view! { <LayoutPanelLeft/> }.into_any(), "Layout Panel Left"),
                (view! { <LayoutPanelTop/> }.into_any(), "Layout Panel Top"),
                (view! { <LayoutTemplate/> }.into_any(), "Layout Template"),
                (view! { <Leaf/> }.into_any(), "Leaf"),
                (view! { <LeafyGreen/> }.into_any(), "Leafy Green"),
                (view! { <Lectern/> }.into_any(), "Lectern"),
                (view! { <LetterText/> }.into_any(), "Letter Text"),
                (view! { <LibraryBig/> }.into_any(), "Library Big"),
                (view! { <Library/> }.into_any(), "Library"),
                (view! { <LifeBuoy/> }.into_any(), "Life Buoy"),
                (view! { <Ligature/> }.into_any(), "Ligature"),
                (view! { <LightbulbOff/> }.into_any(), "Lightbulb Off"),
                (view! { <Lightbulb/> }.into_any(), "Lightbulb"),
                (view! { <Link2Off/> }.into_any(), "Link 2 Off"),
                (view! { <Link2/> }.into_any(), "Link 2"),
                (view! { <Link/> }.into_any(), "Link"),
                (view! { <Linkedin/> }.into_any(), "Linkedin"),
                (view! { <ListCheck/> }.into_any(), "List Check"),
                (view! { <ListChecks/> }.into_any(), "List Checks"),
                (view! { <ListCollapse/> }.into_any(), "List Collapse"),
                (view! { <ListEnd/> }.into_any(), "List End"),
                (view! { <ListFilter/> }.into_any(), "List Filter"),
                (view! { <ListMinus/> }.into_any(), "List Minus"),
                (view! { <ListMusic/> }.into_any(), "List Music"),
                (view! { <ListOrdered/> }.into_any(), "List Ordered"),
                (view! { <ListPlus/> }.into_any(), "List Plus"),
                (view! { <ListRestart/> }.into_any(), "List Restart"),
                (view! { <ListStart/> }.into_any(), "List Start"),
                (view! { <ListTodo/> }.into_any(), "List Todo"),
                (view! { <ListTree/> }.into_any(), "List Tree"),
                (view! { <ListVideo/> }.into_any(), "List Video"),
                (view! { <ListX/> }.into_any(), "List X"),
                (view! { <List/> }.into_any(), "List"),
                (view! { <LoaderCircle/> }.into_any(), "Loader Circle"),
                (view! { <LoaderPinwheel/> }.into_any(), "Loader Pinwheel"),
                (view! { <Loader/> }.into_any(), "Loader"),
                (view! { <LocateFixed/> }.into_any(), "Locate Fixed"),
                (view! { <LocateOff/> }.into_any(), "Locate Off"),
                (view! { <Locate/> }.into_any(), "Locate"),
                (view! { <LockKeyholeOpen/> }.into_any(), "Lock Keyhole Open"),
                (view! { <LockKeyhole/> }.into_any(), "Lock Keyhole"),
                (view! { <LockOpen/> }.into_any(), "Lock Open"),
                (view! { <Lock/> }.into_any(), "Lock"),
                (view! { <LogIn/> }.into_any(), "Log In"),
                (view! { <LogOut/> }.into_any(), "Log Out"),
                (view! { <Logs/> }.into_any(), "Logs"),
                (view! { <Lollipop/> }.into_any(), "Lollipop"),
                (view! { <Luggage/> }.into_any(), "Luggage"),
            ]
            key=|icon| icon.1
            children=move |(icon, name)| {
                view! {
                    <div class="flex flex-wrap items-center gap-4 text-sm">
                        {icon} <span>{name}</span>
                    </div>
                }
            }
        />
    }
}
#[component]
pub fn IconsM() -> impl IntoView {
    view! {
        <For
            each=move || [
                (view! { <Magnet/> }.into_any(), "Magnet"),
                (view! { <MailCheck/> }.into_any(), "Mail Check"),
                (view! { <MailMinus/> }.into_any(), "Mail Minus"),
                (view! { <MailOpen/> }.into_any(), "Mail Open"),
                (view! { <MailPlus/> }.into_any(), "Mail Plus"),
                (view! { <MailQuestion/> }.into_any(), "Mail Question"),
                (view! { <MailSearch/> }.into_any(), "Mail Search"),
                (view! { <MailWarning/> }.into_any(), "Mail Warning"),
                (view! { <MailX/> }.into_any(), "Mail X"),
                (view! { <Mail/> }.into_any(), "Mail"),
                (view! { <Mailbox/> }.into_any(), "Mailbox"),
                (view! { <Mails/> }.into_any(), "Mails"),
                (view! { <MapPinCheckInside/> }.into_any(), "Map Pin Check Inside"),
                (view! { <MapPinCheck/> }.into_any(), "Map Pin Check"),
                (view! { <MapPinHouse/> }.into_any(), "Map Pin House"),
                (view! { <MapPinMinusInside/> }.into_any(), "Map Pin Minus Inside"),
                (view! { <MapPinMinus/> }.into_any(), "Map Pin Minus"),
                (view! { <MapPinOff/> }.into_any(), "Map Pin Off"),
                (view! { <MapPinPlusInside/> }.into_any(), "Map Pin Plus Inside"),
                (view! { <MapPinPlus/> }.into_any(), "Map Pin Plus"),
                (view! { <MapPinXInside/> }.into_any(), "Map Pin X Inside"),
                (view! { <MapPinX/> }.into_any(), "Map Pin X"),
                (view! { <MapPin/> }.into_any(), "Map Pin"),
                (view! { <MapPinned/> }.into_any(), "Map Pinned"),
                (view! { <Map/> }.into_any(), "Map"),
                (view! { <Martini/> }.into_any(), "Martini"),
                (view! { <Maximize2/> }.into_any(), "Maximize 2"),
                (view! { <Maximize/> }.into_any(), "Maximize"),
                (view! { <Medal/> }.into_any(), "Medal"),
                (view! { <MegaphoneOff/> }.into_any(), "Megaphone Off"),
                (view! { <Megaphone/> }.into_any(), "Megaphone"),
                (view! { <Meh/> }.into_any(), "Meh"),
                (view! { <MemoryStick/> }.into_any(), "Memory Stick"),
                (view! { <Menu/> }.into_any(), "Menu"),
                (view! { <Merge/> }.into_any(), "Merge"),
                (view! { <MessageCircleCode/> }.into_any(), "Message Circle Code"),
                (view! { <MessageCircleDashed/> }.into_any(), "Message Circle Dashed"),
                (view! { <MessageCircleHeart/> }.into_any(), "Message Circle Heart"),
                (view! { <MessageCircleMore/> }.into_any(), "Message Circle More"),
                (view! { <MessageCircleOff/> }.into_any(), "Message Circle Off"),
                (view! { <MessageCirclePlus/> }.into_any(), "Message Circle Plus"),
                (view! { <MessageCircleQuestion/> }.into_any(), "Message Circle Question"),
                (view! { <MessageCircleReply/> }.into_any(), "Message Circle Reply"),
                (view! { <MessageCircleWarning/> }.into_any(), "Message Circle Warning"),
                (view! { <MessageCircleX/> }.into_any(), "Message Circle X"),
                (view! { <MessageCircle/> }.into_any(), "Message Circle"),
                (view! { <MessageSquareCode/> }.into_any(), "Message Square Code"),
                (view! { <MessageSquareDashed/> }.into_any(), "Message Square Dashed"),
                (view! { <MessageSquareDiff/> }.into_any(), "Message Square Diff"),
                (view! { <MessageSquareDot/> }.into_any(), "Message Square Dot"),
                (view! { <MessageSquareHeart/> }.into_any(), "Message Square Heart"),
                (view! { <MessageSquareLock/> }.into_any(), "Message Square Lock"),
                (view! { <MessageSquareMore/> }.into_any(), "Message Square More"),
                (view! { <MessageSquareOff/> }.into_any(), "Message Square Off"),
                (view! { <MessageSquarePlus/> }.into_any(), "Message Square Plus"),
                (view! { <MessageSquareQuote/> }.into_any(), "Message Square Quote"),
                (view! { <MessageSquareReply/> }.into_any(), "Message Square Reply"),
                (view! { <MessageSquareShare/> }.into_any(), "Message Square Share"),
                (view! { <MessageSquareText/> }.into_any(), "Message Square Text"),
                (view! { <MessageSquareWarning/> }.into_any(), "Message Square Warning"),
                (view! { <MessageSquareX/> }.into_any(), "Message Square X"),
                (view! { <MessageSquare/> }.into_any(), "Message Square"),
                (view! { <MessagesSquare/> }.into_any(), "Messages Square"),
                (view! { <MicOff/> }.into_any(), "Mic Off"),
                (view! { <MicVocal/> }.into_any(), "Mic Vocal"),
                (view! { <Mic/> }.into_any(), "Mic"),
                (view! { <Microchip/> }.into_any(), "Microchip"),
                (view! { <Microscope/> }.into_any(), "Microscope"),
                (view! { <Microwave/> }.into_any(), "Microwave"),
                (view! { <Milestone/> }.into_any(), "Milestone"),
                (view! { <MilkOff/> }.into_any(), "Milk Off"),
                (view! { <Milk/> }.into_any(), "Milk"),
                (view! { <Minimize2/> }.into_any(), "Minimize 2"),
                (view! { <Minimize/> }.into_any(), "Minimize"),
                (view! { <Minus/> }.into_any(), "Minus"),
                (view! { <MonitorCheck/> }.into_any(), "Monitor Check"),
                (view! { <MonitorCog/> }.into_any(), "Monitor Cog"),
                (view! { <MonitorDot/> }.into_any(), "Monitor Dot"),
                (view! { <MonitorDown/> }.into_any(), "Monitor Down"),
                (view! { <MonitorOff/> }.into_any(), "Monitor Off"),
                (view! { <MonitorPause/> }.into_any(), "Monitor Pause"),
                (view! { <MonitorPlay/> }.into_any(), "Monitor Play"),
                (view! { <MonitorSmartphone/> }.into_any(), "Monitor Smartphone"),
                (view! { <MonitorSpeaker/> }.into_any(), "Monitor Speaker"),
                (view! { <MonitorStop/> }.into_any(), "Monitor Stop"),
                (view! { <MonitorUp/> }.into_any(), "Monitor Up"),
                (view! { <MonitorX/> }.into_any(), "Monitor X"),
                (view! { <Monitor/> }.into_any(), "Monitor"),
                (view! { <MoonStar/> }.into_any(), "Moon Star"),
                (view! { <Moon/> }.into_any(), "Moon"),
                (view! { <MountainSnow/> }.into_any(), "Mountain Snow"),
                (view! { <Mountain/> }.into_any(), "Mountain"),
                (view! { <MouseOff/> }.into_any(), "Mouse Off"),
                (view! { <MousePointer2/> }.into_any(), "Mouse Pointer 2"),
                (view! { <MousePointerBan/> }.into_any(), "Mouse Pointer Ban"),
                (view! { <MousePointerClick/> }.into_any(), "Mouse Pointer Click"),
                (view! { <MousePointer/> }.into_any(), "Mouse Pointer"),
                (view! { <Mouse/> }.into_any(), "Mouse"),
                (view! { <Move3D/> }.into_any(), "Move 3 D"),
                (view! { <MoveDiagonal2/> }.into_any(), "Move Diagonal 2"),
                (view! { <MoveDiagonal/> }.into_any(), "Move Diagonal"),
                (view! { <MoveDownLeft/> }.into_any(), "Move Down Left"),
                (view! { <MoveDownRight/> }.into_any(), "Move Down Right"),
                (view! { <MoveDown/> }.into_any(), "Move Down"),
                (view! { <MoveHorizontal/> }.into_any(), "Move Horizontal"),
                (view! { <MoveLeft/> }.into_any(), "Move Left"),
                (view! { <MoveRight/> }.into_any(), "Move Right"),
                (view! { <MoveUpLeft/> }.into_any(), "Move Up Left"),
                (view! { <MoveUpRight/> }.into_any(), "Move Up Right"),
                (view! { <MoveUp/> }.into_any(), "Move Up"),
                (view! { <MoveVertical/> }.into_any(), "Move Vertical"),
                (view! { <Move/> }.into_any(), "Move"),
                (view! { <Music2/> }.into_any(), "Music 2"),
                (view! { <Music3/> }.into_any(), "Music 3"),
                (view! { <Music4/> }.into_any(), "Music 4"),
                (view! { <Music/> }.into_any(), "Music"),
            ]
            key=|icon| icon.1
            children=move |(icon, name)| {
                view! {
                    <div class="flex flex-wrap items-center gap-4 text-sm">
                        {icon} <span>{name}</span>
                    </div>
                }
            }
        />
    }
}
#[component]
pub fn IconsN() -> impl IntoView {
    view! {
        <For
            each=move || [
                (view! { <Navigation2Off/> }.into_any(), "Navigation 2 Off"),
                (view! { <Navigation2/> }.into_any(), "Navigation 2"),
                (view! { <NavigationOff/> }.into_any(), "Navigation Off"),
                (view! { <Navigation/> }.into_any(), "Navigation"),
                (view! { <Network/> }.into_any(), "Network"),
                (view! { <Newspaper/> }.into_any(), "Newspaper"),
                (view! { <Nfc/> }.into_any(), "Nfc"),
                (view! { <NotebookPen/> }.into_any(), "Notebook Pen"),
                (view! { <NotebookTabs/> }.into_any(), "Notebook Tabs"),
                (view! { <NotebookText/> }.into_any(), "Notebook Text"),
                (view! { <Notebook/> }.into_any(), "Notebook"),
                (view! { <NotepadTextDashed/> }.into_any(), "Notepad Text Dashed"),
                (view! { <NotepadText/> }.into_any(), "Notepad Text"),
                (view! { <NutOff/> }.into_any(), "Nut Off"),
                (view! { <Nut/> }.into_any(), "Nut"),
            ]
            key=|icon| icon.1
            children=move |(icon, name)| {
                view! {
                    <div class="flex flex-wrap items-center gap-4 text-sm">
                        {icon} <span>{name}</span>
                    </div>
                }
            }
        />
    }
}
#[component]
pub fn IconsO() -> impl IntoView {
    view! {
        <For
            each=move || [
                (view! { <OctagonAlert/> }.into_any(), "Octagon Alert"),
                (view! { <OctagonMinus/> }.into_any(), "Octagon Minus"),
                (view! { <OctagonPause/> }.into_any(), "Octagon Pause"),
                (view! { <OctagonX/> }.into_any(), "Octagon X"),
                (view! { <Octagon/> }.into_any(), "Octagon"),
                (view! { <Omega/> }.into_any(), "Omega"),
                (view! { <Option/> }.into_any(), "Option"),
                (view! { <Orbit/> }.into_any(), "Orbit"),
                (view! { <Origami/> }.into_any(), "Origami"),
            ]
            key=|icon| icon.1
            children=move |(icon, name)| {
                view! {
                    <div class="flex flex-wrap items-center gap-4 text-sm">
                        {icon} <span>{name}</span>
                    </div>
                }
            }
        />
    }
}
#[component]
pub fn IconsP() -> impl IntoView {
    view! {
        <For
            each=move || [
                (view! { <Package2/> }.into_any(), "Package 2"),
                (view! { <PackageCheck/> }.into_any(), "Package Check"),
                (view! { <PackageMinus/> }.into_any(), "Package Minus"),
                (view! { <PackageOpen/> }.into_any(), "Package Open"),
                (view! { <PackagePlus/> }.into_any(), "Package Plus"),
                (view! { <PackageSearch/> }.into_any(), "Package Search"),
                (view! { <PackageX/> }.into_any(), "Package X"),
                (view! { <Package/> }.into_any(), "Package"),
                (view! { <PaintBucket/> }.into_any(), "Paint Bucket"),
                (view! { <PaintRoller/> }.into_any(), "Paint Roller"),
                (view! { <PaintbrushVertical/> }.into_any(), "Paintbrush Vertical"),
                (view! { <Paintbrush/> }.into_any(), "Paintbrush"),
                (view! { <Palette/> }.into_any(), "Palette"),
                (view! { <PanelBottomClose/> }.into_any(), "Panel Bottom Close"),
                (view! { <PanelBottomDashed/> }.into_any(), "Panel Bottom Dashed"),
                (view! { <PanelBottomOpen/> }.into_any(), "Panel Bottom Open"),
                (view! { <PanelBottom/> }.into_any(), "Panel Bottom"),
                (view! { <PanelLeftClose/> }.into_any(), "Panel Left Close"),
                (view! { <PanelLeftDashed/> }.into_any(), "Panel Left Dashed"),
                (view! { <PanelLeftOpen/> }.into_any(), "Panel Left Open"),
                (view! { <PanelLeft/> }.into_any(), "Panel Left"),
                (view! { <PanelRightClose/> }.into_any(), "Panel Right Close"),
                (view! { <PanelRightDashed/> }.into_any(), "Panel Right Dashed"),
                (view! { <PanelRightOpen/> }.into_any(), "Panel Right Open"),
                (view! { <PanelRight/> }.into_any(), "Panel Right"),
                (view! { <PanelTopClose/> }.into_any(), "Panel Top Close"),
                (view! { <PanelTopDashed/> }.into_any(), "Panel Top Dashed"),
                (view! { <PanelTopOpen/> }.into_any(), "Panel Top Open"),
                (view! { <PanelTop/> }.into_any(), "Panel Top"),
                (view! { <PanelsLeftBottom/> }.into_any(), "Panels Left Bottom"),
                (view! { <PanelsRightBottom/> }.into_any(), "Panels Right Bottom"),
                (view! { <PanelsTopLeft/> }.into_any(), "Panels Top Left"),
                (view! { <Paperclip/> }.into_any(), "Paperclip"),
                (view! { <Parentheses/> }.into_any(), "Parentheses"),
                (view! { <ParkingMeter/> }.into_any(), "Parking Meter"),
                (view! { <PartyPopper/> }.into_any(), "Party Popper"),
                (view! { <Pause/> }.into_any(), "Pause"),
                (view! { <PawPrint/> }.into_any(), "Paw Print"),
                (view! { <PcCase/> }.into_any(), "Pc Case"),
                (view! { <PenLine/> }.into_any(), "Pen Line"),
                (view! { <PenOff/> }.into_any(), "Pen Off"),
                (view! { <PenTool/> }.into_any(), "Pen Tool"),
                (view! { <Pen/> }.into_any(), "Pen"),
                (view! { <PencilLine/> }.into_any(), "Pencil Line"),
                (view! { <PencilOff/> }.into_any(), "Pencil Off"),
                (view! { <PencilRuler/> }.into_any(), "Pencil Ruler"),
                (view! { <Pencil/> }.into_any(), "Pencil"),
                (view! { <Pentagon/> }.into_any(), "Pentagon"),
                (view! { <Percent/> }.into_any(), "Percent"),
                (view! { <PersonStanding/> }.into_any(), "Person Standing"),
                (view! { <PhilippinePeso/> }.into_any(), "Philippine Peso"),
                (view! { <PhoneCall/> }.into_any(), "Phone Call"),
                (view! { <PhoneForwarded/> }.into_any(), "Phone Forwarded"),
                (view! { <PhoneIncoming/> }.into_any(), "Phone Incoming"),
                (view! { <PhoneMissed/> }.into_any(), "Phone Missed"),
                (view! { <PhoneOff/> }.into_any(), "Phone Off"),
                (view! { <PhoneOutgoing/> }.into_any(), "Phone Outgoing"),
                (view! { <Phone/> }.into_any(), "Phone"),
                (view! { <Pi/> }.into_any(), "Pi"),
                (view! { <Piano/> }.into_any(), "Piano"),
                (view! { <Pickaxe/> }.into_any(), "Pickaxe"),
                (view! { <PictureInPicture2/> }.into_any(), "Picture In Picture 2"),
                (view! { <PictureInPicture/> }.into_any(), "Picture In Picture"),
                (view! { <PiggyBank/> }.into_any(), "Piggy Bank"),
                (view! { <PilcrowLeft/> }.into_any(), "Pilcrow Left"),
                (view! { <PilcrowRight/> }.into_any(), "Pilcrow Right"),
                (view! { <Pilcrow/> }.into_any(), "Pilcrow"),
                (view! { <PillBottle/> }.into_any(), "Pill Bottle"),
                (view! { <Pill/> }.into_any(), "Pill"),
                (view! { <PinOff/> }.into_any(), "Pin Off"),
                (view! { <Pin/> }.into_any(), "Pin"),
                (view! { <Pipette/> }.into_any(), "Pipette"),
                (view! { <Pizza/> }.into_any(), "Pizza"),
                (view! { <PlaneLanding/> }.into_any(), "Plane Landing"),
                (view! { <PlaneTakeoff/> }.into_any(), "Plane Takeoff"),
                (view! { <Plane/> }.into_any(), "Plane"),
                (view! { <Play/> }.into_any(), "Play"),
                (view! { <Plug2/> }.into_any(), "Plug 2"),
                (view! { <PlugZap/> }.into_any(), "Plug Zap"),
                (view! { <Plug/> }.into_any(), "Plug"),
                (view! { <Plus/> }.into_any(), "Plus"),
                (view! { <PocketKnife/> }.into_any(), "Pocket Knife"),
                (view! { <Pocket/> }.into_any(), "Pocket"),
                (view! { <Podcast/> }.into_any(), "Podcast"),
                (view! { <PointerOff/> }.into_any(), "Pointer Off"),
                (view! { <Pointer/> }.into_any(), "Pointer"),
                (view! { <Popcorn/> }.into_any(), "Popcorn"),
                (view! { <Popsicle/> }.into_any(), "Popsicle"),
                (view! { <PoundSterling/> }.into_any(), "Pound Sterling"),
                (view! { <PowerOff/> }.into_any(), "Power Off"),
                (view! { <Power/> }.into_any(), "Power"),
                (view! { <Presentation/> }.into_any(), "Presentation"),
                (view! { <PrinterCheck/> }.into_any(), "Printer Check"),
                (view! { <Printer/> }.into_any(), "Printer"),
                (view! { <Projector/> }.into_any(), "Projector"),
                (view! { <Proportions/> }.into_any(), "Proportions"),
                (view! { <Puzzle/> }.into_any(), "Puzzle"),
                (view! { <Pyramid/> }.into_any(), "Pyramid"),
            ]
            key=|icon| icon.1
            children=move |(icon, name)| {
                view! {
                    <div class="flex flex-wrap items-center gap-4 text-sm">
                        {icon} <span>{name}</span>
                    </div>
                }
            }
        />
    }
}
#[component]
pub fn IconsQ() -> impl IntoView {
    view! {
        <For
            each=move || [
                (view! { <QrCode/> }.into_any(), "Qr Code"),
                (view! { <Quote/> }.into_any(), "Quote"),
            ]
            key=|icon| icon.1
            children=move |(icon, name)| {
                view! {
                    <div class="flex flex-wrap items-center gap-4 text-sm">
                        {icon} <span>{name}</span>
                    </div>
                }
            }
        />
    }
}
#[component]
pub fn IconsR() -> impl IntoView {
    view! {
        <For
            each=move || [
                (view! { <Rabbit/> }.into_any(), "Rabbit"),
                (view! { <Radar/> }.into_any(), "Radar"),
                (view! { <Radiation/> }.into_any(), "Radiation"),
                (view! { <Radical/> }.into_any(), "Radical"),
                (view! { <RadioReceiver/> }.into_any(), "Radio Receiver"),
                (view! { <RadioTower/> }.into_any(), "Radio Tower"),
                (view! { <Radio/> }.into_any(), "Radio"),
                (view! { <Radius/> }.into_any(), "Radius"),
                (view! { <RailSymbol/> }.into_any(), "Rail Symbol"),
                (view! { <Rainbow/> }.into_any(), "Rainbow"),
                (view! { <Rat/> }.into_any(), "Rat"),
                (view! { <Ratio/> }.into_any(), "Ratio"),
                (view! { <ReceiptCent/> }.into_any(), "Receipt Cent"),
                (view! { <ReceiptEuro/> }.into_any(), "Receipt Euro"),
                (view! { <ReceiptIndianRupee/> }.into_any(), "Receipt Indian Rupee"),
                (view! { <ReceiptJapaneseYen/> }.into_any(), "Receipt Japanese Yen"),
                (view! { <ReceiptPoundSterling/> }.into_any(), "Receipt Pound Sterling"),
                (view! { <ReceiptRussianRuble/> }.into_any(), "Receipt Russian Ruble"),
                (view! { <ReceiptSwissFranc/> }.into_any(), "Receipt Swiss Franc"),
                (view! { <ReceiptText/> }.into_any(), "Receipt Text"),
                (view! { <Receipt/> }.into_any(), "Receipt"),
                (view! { <RectangleEllipsis/> }.into_any(), "Rectangle Ellipsis"),
                (view! { <RectangleHorizontal/> }.into_any(), "Rectangle Horizontal"),
                (view! { <RectangleVertical/> }.into_any(), "Rectangle Vertical"),
                (view! { <Recycle/> }.into_any(), "Recycle"),
                (view! { <Redo2/> }.into_any(), "Redo 2"),
                (view! { <RedoDot/> }.into_any(), "Redo Dot"),
                (view! { <Redo/> }.into_any(), "Redo"),
                (view! { <RefreshCcwDot/> }.into_any(), "Refresh Ccw Dot"),
                (view! { <RefreshCcw/> }.into_any(), "Refresh Ccw"),
                (view! { <RefreshCwOff/> }.into_any(), "Refresh Cw Off"),
                (view! { <RefreshCw/> }.into_any(), "Refresh Cw"),
                (view! { <Refrigerator/> }.into_any(), "Refrigerator"),
                (view! { <Regex/> }.into_any(), "Regex"),
                (view! { <RemoveFormatting/> }.into_any(), "Remove Formatting"),
                (view! { <Repeat1/> }.into_any(), "Repeat 1"),
                (view! { <Repeat2/> }.into_any(), "Repeat 2"),
                (view! { <Repeat/> }.into_any(), "Repeat"),
                (view! { <ReplaceAll/> }.into_any(), "Replace All"),
                (view! { <Replace/> }.into_any(), "Replace"),
                (view! { <ReplyAll/> }.into_any(), "Reply All"),
                (view! { <Reply/> }.into_any(), "Reply"),
                (view! { <Rewind/> }.into_any(), "Rewind"),
                (view! { <Ribbon/> }.into_any(), "Ribbon"),
                (view! { <Rocket/> }.into_any(), "Rocket"),
                (view! { <RockingChair/> }.into_any(), "Rocking Chair"),
                (view! { <RollerCoaster/> }.into_any(), "Roller Coaster"),
                (view! { <Rotate3D/> }.into_any(), "Rotate 3 D"),
                (view! { <RotateCcwSquare/> }.into_any(), "Rotate Ccw Square"),
                (view! { <RotateCcw/> }.into_any(), "Rotate Ccw"),
                (view! { <RotateCwSquare/> }.into_any(), "Rotate Cw Square"),
                (view! { <RotateCw/> }.into_any(), "Rotate Cw"),
                (view! { <RouteOff/> }.into_any(), "Route Off"),
                (view! { <Route/> }.into_any(), "Route"),
                (view! { <Router/> }.into_any(), "Router"),
                (view! { <Rows2/> }.into_any(), "Rows 2"),
                (view! { <Rows3/> }.into_any(), "Rows 3"),
                (view! { <Rows4/> }.into_any(), "Rows 4"),
                (view! { <Rss/> }.into_any(), "Rss"),
                (view! { <Ruler/> }.into_any(), "Ruler"),
                (view! { <RussianRuble/> }.into_any(), "Russian Ruble"),
            ]
            key=|icon| icon.1
            children=move |(icon, name)| {
                view! {
                    <div class="flex flex-wrap items-center gap-4 text-sm">
                        {icon} <span>{name}</span>
                    </div>
                }
            }
        />
    }
}
#[component]
pub fn IconsS() -> impl IntoView {
    view! {
        <For
            each=move || [
                (view! { <Sailboat/> }.into_any(), "Sailboat"),
                (view! { <Salad/> }.into_any(), "Salad"),
                (view! { <Sandwich/> }.into_any(), "Sandwich"),
                (view! { <SatelliteDish/> }.into_any(), "Satellite Dish"),
                (view! { <Satellite/> }.into_any(), "Satellite"),
                (view! { <SaveAll/> }.into_any(), "Save All"),
                (view! { <SaveOff/> }.into_any(), "Save Off"),
                (view! { <Save/> }.into_any(), "Save"),
                (view! { <Scale3D/> }.into_any(), "Scale 3 D"),
                (view! { <Scale/> }.into_any(), "Scale"),
                (view! { <Scaling/> }.into_any(), "Scaling"),
                (view! { <ScanBarcode/> }.into_any(), "Scan Barcode"),
                (view! { <ScanEye/> }.into_any(), "Scan Eye"),
                (view! { <ScanFace/> }.into_any(), "Scan Face"),
                (view! { <ScanLine/> }.into_any(), "Scan Line"),
                (view! { <ScanQrCode/> }.into_any(), "Scan Qr Code"),
                (view! { <ScanSearch/> }.into_any(), "Scan Search"),
                (view! { <ScanText/> }.into_any(), "Scan Text"),
                (view! { <Scan/> }.into_any(), "Scan"),
                (view! { <School/> }.into_any(), "School"),
                (view! { <ScissorsLineDashed/> }.into_any(), "Scissors Line Dashed"),
                (view! { <Scissors/> }.into_any(), "Scissors"),
                (view! { <ScreenShareOff/> }.into_any(), "Screen Share Off"),
                (view! { <ScreenShare/> }.into_any(), "Screen Share"),
                (view! { <ScrollText/> }.into_any(), "Scroll Text"),
                (view! { <Scroll/> }.into_any(), "Scroll"),
                (view! { <SearchCheck/> }.into_any(), "Search Check"),
                (view! { <SearchCode/> }.into_any(), "Search Code"),
                (view! { <SearchSlash/> }.into_any(), "Search Slash"),
                (view! { <SearchX/> }.into_any(), "Search X"),
                (view! { <Search/> }.into_any(), "Search"),
                (view! { <Section/> }.into_any(), "Section"),
                (view! { <SendHorizontal/> }.into_any(), "Send Horizontal"),
                (view! { <SendToBack/> }.into_any(), "Send To Back"),
                (view! { <Send/> }.into_any(), "Send"),
                (view! { <SeparatorHorizontal/> }.into_any(), "Separator Horizontal"),
                (view! { <SeparatorVertical/> }.into_any(), "Separator Vertical"),
                (view! { <ServerCog/> }.into_any(), "Server Cog"),
                (view! { <ServerCrash/> }.into_any(), "Server Crash"),
                (view! { <ServerOff/> }.into_any(), "Server Off"),
                (view! { <Server/> }.into_any(), "Server"),
                (view! { <Settings2/> }.into_any(), "Settings 2"),
                (view! { <Settings/> }.into_any(), "Settings"),
                (view! { <Shapes/> }.into_any(), "Shapes"),
                (view! { <Share2/> }.into_any(), "Share 2"),
                (view! { <Share/> }.into_any(), "Share"),
                (view! { <Sheet/> }.into_any(), "Sheet"),
                (view! { <Shell/> }.into_any(), "Shell"),
                (view! { <ShieldAlert/> }.into_any(), "Shield Alert"),
                (view! { <ShieldBan/> }.into_any(), "Shield Ban"),
                (view! { <ShieldCheck/> }.into_any(), "Shield Check"),
                (view! { <ShieldEllipsis/> }.into_any(), "Shield Ellipsis"),
                (view! { <ShieldHalf/> }.into_any(), "Shield Half"),
                (view! { <ShieldMinus/> }.into_any(), "Shield Minus"),
                (view! { <ShieldOff/> }.into_any(), "Shield Off"),
                (view! { <ShieldPlus/> }.into_any(), "Shield Plus"),
                (view! { <ShieldQuestion/> }.into_any(), "Shield Question"),
                (view! { <ShieldX/> }.into_any(), "Shield X"),
                (view! { <Shield/> }.into_any(), "Shield"),
                (view! { <ShipWheel/> }.into_any(), "Ship Wheel"),
                (view! { <Ship/> }.into_any(), "Ship"),
                (view! { <Shirt/> }.into_any(), "Shirt"),
                (view! { <ShoppingBag/> }.into_any(), "Shopping Bag"),
                (view! { <ShoppingBasket/> }.into_any(), "Shopping Basket"),
                (view! { <ShoppingCart/> }.into_any(), "Shopping Cart"),
                (view! { <Shovel/> }.into_any(), "Shovel"),
                (view! { <ShowerHead/> }.into_any(), "Shower Head"),
                (view! { <Shrink/> }.into_any(), "Shrink"),
                (view! { <Shrub/> }.into_any(), "Shrub"),
                (view! { <Shuffle/> }.into_any(), "Shuffle"),
                (view! { <Sigma/> }.into_any(), "Sigma"),
                (view! { <SignalHigh/> }.into_any(), "Signal High"),
                (view! { <SignalLow/> }.into_any(), "Signal Low"),
                (view! { <SignalMedium/> }.into_any(), "Signal Medium"),
                (view! { <SignalZero/> }.into_any(), "Signal Zero"),
                (view! { <Signal/> }.into_any(), "Signal"),
                (view! { <Signature/> }.into_any(), "Signature"),
                (view! { <SignpostBig/> }.into_any(), "Signpost Big"),
                (view! { <Signpost/> }.into_any(), "Signpost"),
                (view! { <Siren/> }.into_any(), "Siren"),
                (view! { <SkipBack/> }.into_any(), "Skip Back"),
                (view! { <SkipForward/> }.into_any(), "Skip Forward"),
                (view! { <Skull/> }.into_any(), "Skull"),
                (view! { <Slack/> }.into_any(), "Slack"),
                (view! { <Slash/> }.into_any(), "Slash"),
                (view! { <Slice/> }.into_any(), "Slice"),
                (view! { <SlidersHorizontal/> }.into_any(), "Sliders Horizontal"),
                (view! { <SlidersVertical/> }.into_any(), "Sliders Vertical"),
                (view! { <SmartphoneCharging/> }.into_any(), "Smartphone Charging"),
                (view! { <SmartphoneNfc/> }.into_any(), "Smartphone Nfc"),
                (view! { <Smartphone/> }.into_any(), "Smartphone"),
                (view! { <SmilePlus/> }.into_any(), "Smile Plus"),
                (view! { <Smile/> }.into_any(), "Smile"),
                (view! { <Snail/> }.into_any(), "Snail"),
                (view! { <Snowflake/> }.into_any(), "Snowflake"),
                (view! { <Sofa/> }.into_any(), "Sofa"),
                (view! { <Soup/> }.into_any(), "Soup"),
                (view! { <Space/> }.into_any(), "Space"),
                (view! { <Spade/> }.into_any(), "Spade"),
                (view! { <Sparkle/> }.into_any(), "Sparkle"),
                (view! { <Sparkles/> }.into_any(), "Sparkles"),
                (view! { <Speaker/> }.into_any(), "Speaker"),
                (view! { <Speech/> }.into_any(), "Speech"),
                (view! { <SpellCheck2/> }.into_any(), "Spell Check 2"),
                (view! { <SpellCheck/> }.into_any(), "Spell Check"),
                (view! { <Spline/> }.into_any(), "Spline"),
                (view! { <Split/> }.into_any(), "Split"),
                (view! { <SprayCan/> }.into_any(), "Spray Can"),
                (view! { <Sprout/> }.into_any(), "Sprout"),
                (view! { <SquareActivity/> }.into_any(), "Square Activity"),
                (view! { <SquareArrowDownLeft/> }.into_any(), "Square Arrow Down Left"),
                (view! { <SquareArrowDownRight/> }.into_any(), "Square Arrow Down Right"),
                (view! { <SquareArrowDown/> }.into_any(), "Square Arrow Down"),
                (view! { <SquareArrowLeft/> }.into_any(), "Square Arrow Left"),
                (view! { <SquareArrowOutDownLeft/> }.into_any(), "Square Arrow Out Down Left"),
                (view! { <SquareArrowOutDownRight/> }.into_any(), "Square Arrow Out Down Right"),
                (view! { <SquareArrowOutUpLeft/> }.into_any(), "Square Arrow Out Up Left"),
                (view! { <SquareArrowOutUpRight/> }.into_any(), "Square Arrow Out Up Right"),
                (view! { <SquareArrowRight/> }.into_any(), "Square Arrow Right"),
                (view! { <SquareArrowUpLeft/> }.into_any(), "Square Arrow Up Left"),
                (view! { <SquareArrowUpRight/> }.into_any(), "Square Arrow Up Right"),
                (view! { <SquareArrowUp/> }.into_any(), "Square Arrow Up"),
                (view! { <SquareAsterisk/> }.into_any(), "Square Asterisk"),
                (
                    view! { <SquareBottomDashedScissors/> }.into_any(),
                    "Square Bottom Dashed Scissors",
                ),
                (view! { <SquareChartGantt/> }.into_any(), "Square Chart Gantt"),
                (view! { <SquareCheckBig/> }.into_any(), "Square Check Big"),
                (view! { <SquareCheck/> }.into_any(), "Square Check"),
                (view! { <SquareChevronDown/> }.into_any(), "Square Chevron Down"),
                (view! { <SquareChevronLeft/> }.into_any(), "Square Chevron Left"),
                (view! { <SquareChevronRight/> }.into_any(), "Square Chevron Right"),
                (view! { <SquareChevronUp/> }.into_any(), "Square Chevron Up"),
                (view! { <SquareCode/> }.into_any(), "Square Code"),
                (view! { <SquareDashedBottomCode/> }.into_any(), "Square Dashed Bottom Code"),
                (view! { <SquareDashedBottom/> }.into_any(), "Square Dashed Bottom"),
                (view! { <SquareDashedKanban/> }.into_any(), "Square Dashed Kanban"),
                (view! { <SquareDashedMousePointer/> }.into_any(), "Square Dashed Mouse Pointer"),
                (view! { <SquareDashed/> }.into_any(), "Square Dashed"),
                (view! { <SquareDivide/> }.into_any(), "Square Divide"),
                (view! { <SquareDot/> }.into_any(), "Square Dot"),
                (view! { <SquareEqual/> }.into_any(), "Square Equal"),
                (view! { <SquareFunction/> }.into_any(), "Square Function"),
                (view! { <SquareKanban/> }.into_any(), "Square Kanban"),
                (view! { <SquareLibrary/> }.into_any(), "Square Library"),
                (view! { <SquareM/> }.into_any(), "Square M"),
                (view! { <SquareMenu/> }.into_any(), "Square Menu"),
                (view! { <SquareMinus/> }.into_any(), "Square Minus"),
                (view! { <SquareMousePointer/> }.into_any(), "Square Mouse Pointer"),
                (view! { <SquareParkingOff/> }.into_any(), "Square Parking Off"),
                (view! { <SquareParking/> }.into_any(), "Square Parking"),
                (view! { <SquarePen/> }.into_any(), "Square Pen"),
                (view! { <SquarePercent/> }.into_any(), "Square Percent"),
                (view! { <SquarePi/> }.into_any(), "Square Pi"),
                (view! { <SquarePilcrow/> }.into_any(), "Square Pilcrow"),
                (view! { <SquarePlay/> }.into_any(), "Square Play"),
                (view! { <SquarePlus/> }.into_any(), "Square Plus"),
                (view! { <SquarePower/> }.into_any(), "Square Power"),
                (view! { <SquareRadical/> }.into_any(), "Square Radical"),
                (view! { <SquareScissors/> }.into_any(), "Square Scissors"),
                (view! { <SquareSigma/> }.into_any(), "Square Sigma"),
                (view! { <SquareSlash/> }.into_any(), "Square Slash"),
                (view! { <SquareSplitHorizontal/> }.into_any(), "Square Split Horizontal"),
                (view! { <SquareSplitVertical/> }.into_any(), "Square Split Vertical"),
                (view! { <SquareSquare/> }.into_any(), "Square Square"),
                (view! { <SquareStack/> }.into_any(), "Square Stack"),
                (view! { <SquareTerminal/> }.into_any(), "Square Terminal"),
                (view! { <SquareUserRound/> }.into_any(), "Square User Round"),
                (view! { <SquareUser/> }.into_any(), "Square User"),
                (view! { <SquareX/> }.into_any(), "Square X"),
                (view! { <Square/> }.into_any(), "Square"),
                (view! { <Squircle/> }.into_any(), "Squircle"),
                (view! { <Squirrel/> }.into_any(), "Squirrel"),
                (view! { <Stamp/> }.into_any(), "Stamp"),
                (view! { <StarHalf/> }.into_any(), "Star Half"),
                (view! { <StarOff/> }.into_any(), "Star Off"),
                (view! { <Star/> }.into_any(), "Star"),
                (view! { <StepBack/> }.into_any(), "Step Back"),
                (view! { <StepForward/> }.into_any(), "Step Forward"),
                (view! { <Stethoscope/> }.into_any(), "Stethoscope"),
                (view! { <Sticker/> }.into_any(), "Sticker"),
                (view! { <StickyNote/> }.into_any(), "Sticky Note"),
                (view! { <Store/> }.into_any(), "Store"),
                (view! { <StretchHorizontal/> }.into_any(), "Stretch Horizontal"),
                (view! { <StretchVertical/> }.into_any(), "Stretch Vertical"),
                (view! { <Strikethrough/> }.into_any(), "Strikethrough"),
                (view! { <Subscript/> }.into_any(), "Subscript"),
                (view! { <SunDim/> }.into_any(), "Sun Dim"),
                (view! { <SunMedium/> }.into_any(), "Sun Medium"),
                (view! { <SunMoon/> }.into_any(), "Sun Moon"),
                (view! { <SunSnow/> }.into_any(), "Sun Snow"),
                (view! { <Sun/> }.into_any(), "Sun"),
                (view! { <Sunrise/> }.into_any(), "Sunrise"),
                (view! { <Sunset/> }.into_any(), "Sunset"),
                (view! { <Superscript/> }.into_any(), "Superscript"),
                (view! { <SwatchBook/> }.into_any(), "Swatch Book"),
                (view! { <SwissFranc/> }.into_any(), "Swiss Franc"),
                (view! { <SwitchCamera/> }.into_any(), "Switch Camera"),
                (view! { <Sword/> }.into_any(), "Sword"),
                (view! { <Swords/> }.into_any(), "Swords"),
                (view! { <Syringe/> }.into_any(), "Syringe"),
            ]
            key=|icon| icon.1
            children=move |(icon, name)| {
                view! {
                    <div class="flex flex-wrap items-center gap-4 text-sm">
                        {icon} <span>{name}</span>
                    </div>
                }
            }
        />
    }
}
#[component]
pub fn IconsT() -> impl IntoView {
    view! {
        <For
            each=move || [
                (view! { <Table2/> }.into_any(), "Table 2"),
                (view! { <TableCellsMerge/> }.into_any(), "Table Cells Merge"),
                (view! { <TableCellsSplit/> }.into_any(), "Table Cells Split"),
                (view! { <TableColumnsSplit/> }.into_any(), "Table Columns Split"),
                (view! { <TableOfContents/> }.into_any(), "Table Of Contents"),
                (view! { <TableProperties/> }.into_any(), "Table Properties"),
                (view! { <TableRowsSplit/> }.into_any(), "Table Rows Split"),
                (view! { <Table/> }.into_any(), "Table"),
                (view! { <TabletSmartphone/> }.into_any(), "Tablet Smartphone"),
                (view! { <Tablet/> }.into_any(), "Tablet"),
                (view! { <Tablets/> }.into_any(), "Tablets"),
                (view! { <Tag/> }.into_any(), "Tag"),
                (view! { <Tags/> }.into_any(), "Tags"),
                (view! { <Tally1/> }.into_any(), "Tally 1"),
                (view! { <Tally2/> }.into_any(), "Tally 2"),
                (view! { <Tally3/> }.into_any(), "Tally 3"),
                (view! { <Tally4/> }.into_any(), "Tally 4"),
                (view! { <Tally5/> }.into_any(), "Tally 5"),
                (view! { <Tangent/> }.into_any(), "Tangent"),
                (view! { <Target/> }.into_any(), "Target"),
                (view! { <Telescope/> }.into_any(), "Telescope"),
                (view! { <TentTree/> }.into_any(), "Tent Tree"),
                (view! { <Tent/> }.into_any(), "Tent"),
                (view! { <Terminal/> }.into_any(), "Terminal"),
                (view! { <TestTubeDiagonal/> }.into_any(), "Test Tube Diagonal"),
                (view! { <TestTube/> }.into_any(), "Test Tube"),
                (view! { <TestTubes/> }.into_any(), "Test Tubes"),
                (view! { <TextCursorInput/> }.into_any(), "Text Cursor Input"),
                (view! { <TextCursor/> }.into_any(), "Text Cursor"),
                (view! { <TextQuote/> }.into_any(), "Text Quote"),
                (view! { <TextSearch/> }.into_any(), "Text Search"),
                (view! { <TextSelect/> }.into_any(), "Text Select"),
                (view! { <Text/> }.into_any(), "Text"),
                (view! { <Theater/> }.into_any(), "Theater"),
                (view! { <ThermometerSnowflake/> }.into_any(), "Thermometer Snowflake"),
                (view! { <ThermometerSun/> }.into_any(), "Thermometer Sun"),
                (view! { <Thermometer/> }.into_any(), "Thermometer"),
                (view! { <ThumbsDown/> }.into_any(), "Thumbs Down"),
                (view! { <ThumbsUp/> }.into_any(), "Thumbs Up"),
                (view! { <TicketCheck/> }.into_any(), "Ticket Check"),
                (view! { <TicketMinus/> }.into_any(), "Ticket Minus"),
                (view! { <TicketPercent/> }.into_any(), "Ticket Percent"),
                (view! { <TicketPlus/> }.into_any(), "Ticket Plus"),
                (view! { <TicketSlash/> }.into_any(), "Ticket Slash"),
                (view! { <TicketX/> }.into_any(), "Ticket X"),
                (view! { <Ticket/> }.into_any(), "Ticket"),
                (view! { <TicketsPlane/> }.into_any(), "Tickets Plane"),
                (view! { <Tickets/> }.into_any(), "Tickets"),
                (view! { <TimerOff/> }.into_any(), "Timer Off"),
                (view! { <TimerReset/> }.into_any(), "Timer Reset"),
                (view! { <Timer/> }.into_any(), "Timer"),
                (view! { <ToggleLeft/> }.into_any(), "Toggle Left"),
                (view! { <ToggleRight/> }.into_any(), "Toggle Right"),
                (view! { <Toilet/> }.into_any(), "Toilet"),
                (view! { <Tornado/> }.into_any(), "Tornado"),
                (view! { <Torus/> }.into_any(), "Torus"),
                (view! { <TouchpadOff/> }.into_any(), "Touchpad Off"),
                (view! { <Touchpad/> }.into_any(), "Touchpad"),
                (view! { <TowerControl/> }.into_any(), "Tower Control"),
                (view! { <ToyBrick/> }.into_any(), "Toy Brick"),
                (view! { <Tractor/> }.into_any(), "Tractor"),
                (view! { <TrafficCone/> }.into_any(), "Traffic Cone"),
                (view! { <TrainFrontTunnel/> }.into_any(), "Train Front Tunnel"),
                (view! { <TrainFront/> }.into_any(), "Train Front"),
                (view! { <TrainTrack/> }.into_any(), "Train Track"),
                (view! { <TramFront/> }.into_any(), "Tram Front"),
                (view! { <Trash2/> }.into_any(), "Trash 2"),
                (view! { <Trash/> }.into_any(), "Trash"),
                (view! { <TreeDeciduous/> }.into_any(), "Tree Deciduous"),
                (view! { <TreePalm/> }.into_any(), "Tree Palm"),
                (view! { <TreePine/> }.into_any(), "Tree Pine"),
                (view! { <Trees/> }.into_any(), "Trees"),
                (view! { <Trello/> }.into_any(), "Trello"),
                (view! { <TrendingDown/> }.into_any(), "Trending Down"),
                (view! { <TrendingUpDown/> }.into_any(), "Trending Up Down"),
                (view! { <TrendingUp/> }.into_any(), "Trending Up"),
                (view! { <TriangleAlert/> }.into_any(), "Triangle Alert"),
                (view! { <TriangleRight/> }.into_any(), "Triangle Right"),
                (view! { <Triangle/> }.into_any(), "Triangle"),
                (view! { <Trophy/> }.into_any(), "Trophy"),
                (view! { <Truck/> }.into_any(), "Truck"),
                (view! { <Turtle/> }.into_any(), "Turtle"),
                (view! { <TvMinimalPlay/> }.into_any(), "Tv Minimal Play"),
                (view! { <TvMinimal/> }.into_any(), "Tv Minimal"),
                (view! { <Tv/> }.into_any(), "Tv"),
                (view! { <Twitch/> }.into_any(), "Twitch"),
                (view! { <Twitter/> }.into_any(), "Twitter"),
                (view! { <TypeOutline/> }.into_any(), "Type Outline"),
                (view! { <Type/> }.into_any(), "Type"),
            ]
            key=|icon| icon.1
            children=move |(icon, name)| {
                view! {
                    <div class="flex flex-wrap items-center gap-4 text-sm">
                        {icon} <span>{name}</span>
                    </div>
                }
            }
        />
    }
}
#[component]
pub fn IconsU() -> impl IntoView {
    view! {
        <For
            each=move || [
                (view! { <UmbrellaOff/> }.into_any(), "Umbrella Off"),
                (view! { <Umbrella/> }.into_any(), "Umbrella"),
                (view! { <Underline/> }.into_any(), "Underline"),
                (view! { <Undo2/> }.into_any(), "Undo 2"),
                (view! { <UndoDot/> }.into_any(), "Undo Dot"),
                (view! { <Undo/> }.into_any(), "Undo"),
                (view! { <UnfoldHorizontal/> }.into_any(), "Unfold Horizontal"),
                (view! { <UnfoldVertical/> }.into_any(), "Unfold Vertical"),
                (view! { <Ungroup/> }.into_any(), "Ungroup"),
                (view! { <University/> }.into_any(), "University"),
                (view! { <Unlink2/> }.into_any(), "Unlink 2"),
                (view! { <Unlink/> }.into_any(), "Unlink"),
                (view! { <Unplug/> }.into_any(), "Unplug"),
                (view! { <Upload/> }.into_any(), "Upload"),
                (view! { <Usb/> }.into_any(), "Usb"),
                (view! { <UserCheck/> }.into_any(), "User Check"),
                (view! { <UserCog/> }.into_any(), "User Cog"),
                (view! { <UserMinus/> }.into_any(), "User Minus"),
                (view! { <UserPen/> }.into_any(), "User Pen"),
                (view! { <UserPlus/> }.into_any(), "User Plus"),
                (view! { <UserRoundCheck/> }.into_any(), "User Round Check"),
                (view! { <UserRoundCog/> }.into_any(), "User Round Cog"),
                (view! { <UserRoundMinus/> }.into_any(), "User Round Minus"),
                (view! { <UserRoundPen/> }.into_any(), "User Round Pen"),
                (view! { <UserRoundPlus/> }.into_any(), "User Round Plus"),
                (view! { <UserRoundSearch/> }.into_any(), "User Round Search"),
                (view! { <UserRoundX/> }.into_any(), "User Round X"),
                (view! { <UserRound/> }.into_any(), "User Round"),
                (view! { <UserSearch/> }.into_any(), "User Search"),
                (view! { <UserX/> }.into_any(), "User X"),
                (view! { <User/> }.into_any(), "User"),
                (view! { <UsersRound/> }.into_any(), "Users Round"),
                (view! { <Users/> }.into_any(), "Users"),
                (view! { <UtensilsCrossed/> }.into_any(), "Utensils Crossed"),
                (view! { <Utensils/> }.into_any(), "Utensils"),
                (view! { <UtilityPole/> }.into_any(), "Utility Pole"),
            ]
            key=|icon| icon.1
            children=move |(icon, name)| {
                view! {
                    <div class="flex flex-wrap items-center gap-4 text-sm">
                        {icon} <span>{name}</span>
                    </div>
                }
            }
        />
    }
}
#[component]
pub fn IconsV() -> impl IntoView {
    view! {
        <For
            each=move || [
                (view! { <Variable/> }.into_any(), "Variable"),
                (view! { <Vault/> }.into_any(), "Vault"),
                (view! { <Vegan/> }.into_any(), "Vegan"),
                (view! { <VenetianMask/> }.into_any(), "Venetian Mask"),
                (view! { <VibrateOff/> }.into_any(), "Vibrate Off"),
                (view! { <Vibrate/> }.into_any(), "Vibrate"),
                (view! { <VideoOff/> }.into_any(), "Video Off"),
                (view! { <Video/> }.into_any(), "Video"),
                (view! { <Videotape/> }.into_any(), "Videotape"),
                (view! { <View/> }.into_any(), "View"),
                (view! { <Voicemail/> }.into_any(), "Voicemail"),
                (view! { <Volleyball/> }.into_any(), "Volleyball"),
                (view! { <Volume1/> }.into_any(), "Volume 1"),
                (view! { <Volume2/> }.into_any(), "Volume 2"),
                (view! { <VolumeOff/> }.into_any(), "Volume Off"),
                (view! { <VolumeX/> }.into_any(), "Volume X"),
                (view! { <Volume/> }.into_any(), "Volume"),
                (view! { <Vote/> }.into_any(), "Vote"),
            ]
            key=|icon| icon.1
            children=move |(icon, name)| {
                view! {
                    <div class="flex flex-wrap items-center gap-4 text-sm">
                        {icon} <span>{name}</span>
                    </div>
                }
            }
        />
    }
}
#[component]
pub fn IconsW() -> impl IntoView {
    view! {
        <For
            each=move || [
                (view! { <WalletCards/> }.into_any(), "Wallet Cards"),
                (view! { <WalletMinimal/> }.into_any(), "Wallet Minimal"),
                (view! { <Wallet/> }.into_any(), "Wallet"),
                (view! { <Wallpaper/> }.into_any(), "Wallpaper"),
                (view! { <WandSparkles/> }.into_any(), "Wand Sparkles"),
                (view! { <Wand/> }.into_any(), "Wand"),
                (view! { <Warehouse/> }.into_any(), "Warehouse"),
                (view! { <WashingMachine/> }.into_any(), "Washing Machine"),
                (view! { <Watch/> }.into_any(), "Watch"),
                (view! { <Waves/> }.into_any(), "Waves"),
                (view! { <Waypoints/> }.into_any(), "Waypoints"),
                (view! { <Webcam/> }.into_any(), "Webcam"),
                (view! { <WebhookOff/> }.into_any(), "Webhook Off"),
                (view! { <Webhook/> }.into_any(), "Webhook"),
                (view! { <Weight/> }.into_any(), "Weight"),
                (view! { <WheatOff/> }.into_any(), "Wheat Off"),
                (view! { <Wheat/> }.into_any(), "Wheat"),
                (view! { <WholeWord/> }.into_any(), "Whole Word"),
                (view! { <WifiHigh/> }.into_any(), "Wifi High"),
                (view! { <WifiLow/> }.into_any(), "Wifi Low"),
                (view! { <WifiOff/> }.into_any(), "Wifi Off"),
                (view! { <WifiZero/> }.into_any(), "Wifi Zero"),
                (view! { <Wifi/> }.into_any(), "Wifi"),
                (view! { <WindArrowDown/> }.into_any(), "Wind Arrow Down"),
                (view! { <Wind/> }.into_any(), "Wind"),
                (view! { <WineOff/> }.into_any(), "Wine Off"),
                (view! { <Wine/> }.into_any(), "Wine"),
                (view! { <Workflow/> }.into_any(), "Workflow"),
                (view! { <Worm/> }.into_any(), "Worm"),
                (view! { <WrapText/> }.into_any(), "Wrap Text"),
                (view! { <Wrench/> }.into_any(), "Wrench"),
            ]
            key=|icon| icon.1
            children=move |(icon, name)| {
                view! {
                    <div class="flex flex-wrap items-center gap-4 text-sm">
                        {icon} <span>{name}</span>
                    </div>
                }
            }
        />
    }
}
#[component]
pub fn IconsX() -> impl IntoView {
    view! {
        <For
            each=move || [(view! { <X/> }.into_any(), "X")]
            key=|icon| icon.1
            children=move |(icon, name)| {
                view! {
                    <div class="flex flex-wrap items-center gap-4 text-sm">
                        {icon} <span>{name}</span>
                    </div>
                }
            }
        />
    }
}
#[component]
pub fn IconsY() -> impl IntoView {
    view! {
        <For
            each=move || [(view! { <Youtube/> }.into_any(), "Youtube")]
            key=|icon| icon.1
            children=move |(icon, name)| {
                view! {
                    <div class="flex flex-wrap items-center gap-4 text-sm">
                        {icon} <span>{name}</span>
                    </div>
                }
            }
        />
    }
}
#[component]
pub fn IconsZ() -> impl IntoView {
    view! {
        <For
            each=move || [
                (view! { <ZapOff/> }.into_any(), "Zap Off"),
                (view! { <Zap/> }.into_any(), "Zap"),
                (view! { <ZoomIn/> }.into_any(), "Zoom In"),
                (view! { <ZoomOut/> }.into_any(), "Zoom Out"),
            ]
            key=|icon| icon.1
            children=move |(icon, name)| {
                view! {
                    <div class="flex flex-wrap items-center gap-4 text-sm">
                        {icon} <span>{name}</span>
                    </div>
                }
            }
        />
    }
}
