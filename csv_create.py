# %%

import csv
import logging
import random
from datetime import datetime, timedelta
from typing import Any, List, Optional, Sequence, Tuple

log = logging.getLogger(__name__)


def export_tuple_to_csv(
    data: List[Tuple[Any, ...]],
    filepath: str,
    delimiter: str = ",",
    fieldnames: Optional[Sequence[str]] = None,
    header: Optional[str] = None,
):
    """Output a list of tuples to a csv file.

    filepath includes filename that ends on ".csv".
    """

    if not data:
        log.warning("No data to export")
        return

    with open(filepath, "w", newline="") as file:
        writer = csv.writer(file, delimiter=delimiter)
        if header:
            writer.writerow((header,))
        if fieldnames:
            writer.writerow(fieldnames)
        writer.writerows(data)


prices_dt = [
    # Monthly prices
    (datetime(2024, 1, 1, 0, 0), datetime(2024, 2, 1, 0, 0), 1),
    (datetime(2024, 2, 1, 0, 0), datetime(2024, 3, 1, 0, 0), 3),
    # 15-minute prices
    (datetime(2024, 3, 1, 0, 0), datetime(2024, 3, 1, 0, 15), 5),
    (datetime(2024, 3, 1, 0, 15), datetime(2024, 3, 1, 0, 30), 7),
    (datetime(2024, 3, 1, 0, 30), datetime(2024, 3, 1, 0, 45), 13),
    (datetime(2024, 3, 1, 0, 45), datetime(2024, 3, 1, 1, 0), 17),
    (datetime(2024, 3, 1, 1, 0), datetime(2024, 3, 1, 1, 15), 19),
    (datetime(2024, 3, 1, 1, 15), datetime(2024, 3, 1, 1, 30), 23),
    (datetime(2024, 3, 1, 1, 30), datetime(2024, 3, 1, 1, 45), 29),
    (datetime(2024, 3, 1, 1, 45), datetime(2024, 3, 1, 2, 0), 31),
    (datetime(2024, 3, 1, 2, 0), datetime(2024, 3, 1, 2, 15), 43),
    (datetime(2024, 3, 1, 2, 15), datetime(2024, 3, 1, 2, 30), 47),
    (datetime(2024, 3, 1, 2, 30), datetime(2024, 3, 1, 2, 45), 5),
    (datetime(2024, 3, 1, 2, 45), datetime(2024, 3, 1, 3, 0), 7),
    # Hourly prices
    (datetime(2024, 3, 1, 3, 0), datetime(2024, 3, 1, 4, 0), 2),
    (datetime(2024, 3, 1, 4, 0), datetime(2024, 3, 1, 5, 0), 4),
    (datetime(2024, 3, 1, 5, 0), datetime(2024, 3, 1, 6, 0), 6),
    (datetime(2024, 3, 1, 6, 0), datetime(2024, 3, 1, 7, 0), 8),
    (datetime(2024, 3, 1, 7, 0), datetime(2024, 3, 1, 8, 0), 10),
    (datetime(2024, 3, 1, 8, 0), datetime(2024, 3, 1, 9, 0), 12),
    (datetime(2024, 3, 1, 9, 0), datetime(2024, 3, 1, 10, 0), 14),
    (datetime(2024, 3, 1, 10, 0), datetime(2024, 3, 1, 11, 0), 16),
    (datetime(2024, 3, 1, 11, 0), datetime(2024, 3, 1, 12, 0), 18),
    (datetime(2024, 3, 1, 12, 0), datetime(2024, 3, 1, 13, 0), 20),
    (datetime(2024, 3, 1, 13, 0), datetime(2024, 3, 1, 14, 0), 22),
    (datetime(2024, 3, 1, 14, 0), datetime(2024, 3, 1, 15, 0), 24),
    (datetime(2024, 3, 1, 15, 0), datetime(2024, 3, 1, 16, 0), 26),
    (datetime(2024, 3, 1, 16, 0), datetime(2024, 3, 1, 17, 0), 28),
    (datetime(2024, 3, 1, 17, 0), datetime(2024, 3, 1, 18, 0), 30),
    (datetime(2024, 3, 1, 18, 0), datetime(2024, 3, 1, 19, 0), 32),
    (datetime(2024, 3, 1, 19, 0), datetime(2024, 3, 1, 20, 0), 34),
    (datetime(2024, 3, 1, 20, 0), datetime(2024, 3, 1, 21, 0), 36),
    (datetime(2024, 3, 1, 21, 0), datetime(2024, 3, 1, 22, 0), 38),
    (datetime(2024, 3, 1, 22, 0), datetime(2024, 3, 1, 23, 0), 40),
    (datetime(2024, 3, 1, 23, 0), datetime(2024, 3, 2, 0, 0), 42),
    # daily prices
    (datetime(2024, 3, 2, 0, 0), datetime(2024, 3, 3, 0, 0), 11),
    (datetime(2024, 3, 3, 0, 0), datetime(2024, 3, 4, 0, 0), 12),
    (datetime(2024, 3, 4, 0, 0), datetime(2024, 3, 5, 0, 0), 13),
    (datetime(2024, 3, 5, 0, 0), datetime(2024, 3, 6, 0, 0), 14),
    (datetime(2024, 3, 6, 0, 0), datetime(2024, 3, 7, 0, 0), 15),
    (datetime(2024, 3, 7, 0, 0), datetime(2024, 3, 8, 0, 0), 16),
    (datetime(2024, 3, 8, 0, 0), datetime(2024, 3, 9, 0, 0), 17),
    (datetime(2024, 3, 9, 0, 0), datetime(2024, 3, 10, 0, 0), 18),
    (datetime(2024, 3, 10, 0, 0), datetime(2024, 3, 11, 0, 0), 19),
    (datetime(2024, 3, 11, 0, 0), datetime(2024, 3, 12, 0, 0), 20),
    (datetime(2024, 3, 12, 0, 0), datetime(2024, 3, 13, 0, 0), 21),
    (datetime(2024, 3, 13, 0, 0), datetime(2024, 3, 14, 0, 0), 22),
    (datetime(2024, 3, 14, 0, 0), datetime(2024, 3, 15, 0, 0), 23),
    (datetime(2024, 3, 15, 0, 0), datetime(2024, 3, 16, 0, 0), 24),
    (datetime(2024, 3, 16, 0, 0), datetime(2024, 3, 17, 0, 0), 25),
    (datetime(2024, 3, 17, 0, 0), datetime(2024, 3, 18, 0, 0), 26),
    (datetime(2024, 3, 18, 0, 0), datetime(2024, 3, 19, 0, 0), 27),
    (datetime(2024, 3, 19, 0, 0), datetime(2024, 3, 20, 0, 0), 28),
    (datetime(2024, 3, 20, 0, 0), datetime(2024, 3, 21, 0, 0), 29),
    (datetime(2024, 3, 21, 0, 0), datetime(2024, 3, 22, 0, 0), 30),
    (datetime(2024, 3, 22, 0, 0), datetime(2024, 3, 23, 0, 0), 31),
    (datetime(2024, 3, 23, 0, 0), datetime(2024, 3, 24, 0, 0), 32),
    (datetime(2024, 3, 24, 0, 0), datetime(2024, 3, 25, 0, 0), 33),
    (datetime(2024, 3, 25, 0, 0), datetime(2024, 3, 26, 0, 0), 34),
    (datetime(2024, 3, 26, 0, 0), datetime(2024, 3, 27, 0, 0), 35),
    (datetime(2024, 3, 27, 0, 0), datetime(2024, 3, 28, 0, 0), 36),
    (datetime(2024, 3, 28, 0, 0), datetime(2024, 3, 29, 0, 0), 37),
    (datetime(2024, 3, 29, 0, 0), datetime(2024, 3, 30, 0, 0), 38),
    (datetime(2024, 3, 30, 0, 0), datetime(2024, 3, 31, 0, 0), 39),
    (datetime(2024, 3, 31, 0, 0), datetime(2024, 4, 1, 0, 0), 40),
    # Quarterly
    (datetime(2024, 4, 1, 0, 0), datetime(2024, 7, 1, 0, 0), 100),
    (datetime(2024, 7, 1, 0, 0), datetime(2024, 10, 1, 0, 0), 110),
    (datetime(2024, 10, 1, 0, 0), datetime(2025, 1, 1, 0, 0), 120),
]

prices_dt += [
    (
        datetime(2025, 1, 1, 0, 0) + timedelta(days=i),
        datetime(2025, 1, 1, 0, 0) + timedelta(days=i + 1),
        random.randint(0, 1000),
    )
    for i in range(0, 365 + 365)
]

o = [(i[0].timestamp(), i[1].timestamp(), i[2]) for i in prices_dt]

export_tuple_to_csv(
    o, "curve_cont.csv", ",", ["delivery_start_utc", "delivery_end_utc", "value"]
)


# %%


delivs = []
for i in range(400_000):
    new_day = random.randint(0, 200)
    start = datetime(2025, 1, 1, 0, 0) + timedelta(days=new_day)
    end = start + timedelta(days=1)
    value = max(new_day, 100)

    delivs.append((start.timestamp(), end.timestamp(), value))

    start += timedelta(minutes=15 * new_day)
    end = start + timedelta(minutes=15)
    value = max(new_day * 7, 7)

    delivs.append((start.timestamp(), end.timestamp(), value))


for i in range(200_000):
    new_day = random.randint(1, 200)
    start = datetime(2025, 1, 1, 0, 0) + timedelta(days=new_day)
    end = start + timedelta(days=new_day)
    value = max(new_day, 100)

    delivs.append((start.timestamp(), end.timestamp(), value))


export_tuple_to_csv(
    delivs, "positions.csv", ",", ["delivery_start", "delivery_end", "value"]
)
