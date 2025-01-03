# kg_EveryTickSOHLCP
#
# Strategy to capture every bar OHLC and P, the previous close.
# Useful for exporting data from TOS into a CSV file for further processing.
#
# Author: Kory Gill, @korygill
declare upper;
declare once_per_bar;

input startTime = 820; #hint startTime: start time in EST 24 - hour time
input endTime = 1600; #hint endTime: end time in EST 24 - hour time

def adjStartTime = startTime;# - 1;
def adjEndTime = endTime;# - 1;

def agg = GetAggregationPeriod();

# we use a 1 bar offset to get orders to line up, so adjust for that here
def marketOpen = if agg >= AggregationPeriod.DAY then 1 else if SecondsTillTime(adjEndTime) >= 60 and SecondsFromTime(adjStartTime) >= -60 then 1 else 0;

AddOrder(OrderType.BUY_TO_OPEN,
	marketOpen,
	low,
	1,
	Color.White,
	Color.White,
	name = "SOHLCP|" + GetSymbol() + "|" + open[-1] + "|" + high[-1] + "|" + low[-1] + "|" + close[-1] + "|" + close);
AddOrder(OrderType.SELL_TO_CLOSE, marketOpen, high, 1, Color.White, Color.White, name = "SellClose");