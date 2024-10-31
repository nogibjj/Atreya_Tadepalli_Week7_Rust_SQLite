import sqlite3
import csv
import os
"""
Transforms and Loads data into the local SQLite3 database
Example:
dispatching_base_number,date,active_vehicles,trips
"""


#load the csv file and insert into a new sqlite3 database
def load(dataset="data/Uber-Jan-Feb-FOIL.csv"):
    """"Transforms and Loads data into the local SQLite3 database"""

    #prints the full working directory and path
    print(os.getcwd())
    payload = csv.reader(open(dataset, newline=''), delimiter=',')
    next(payload)
    conn = sqlite3.connect('Ubertrips.db')
    c = conn.cursor()
    c.execute("DROP TABLE IF EXISTS Ubertrips")
    c.execute("CREATE TABLE Ubertrips \
    (dispatching_base_number,date,active_vehicles,trips)")
    #insert
    c.executemany("INSERT INTO Ubertrips VALUES (?,?, ?, ?)", payload)
    conn.commit()
    conn.close()
    return "Ubertrips.db"

if __name__ == "__main__":
    load()

