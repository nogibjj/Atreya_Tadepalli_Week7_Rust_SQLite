"""Query the database"""

import sqlite3

test_date=2/24/2015
test_dispatch="B05689"
test_active=362
test_trips=1546

def create_query(dispatch_number,date,vehicles,trips1):
    """Query the database to insert a new row within the Ubertrips table"""
    conn = sqlite3.connect("Ubertrips.db")
    cursor = conn.cursor()
    """Create a new entry"""
    cursor.execute(
        """
        INSERT INTO Ubertrips 
        (dispatching_base_number,date,active_vehicles,trips) 
        VALUES (?, ?, ?, ?)
        """,
        (dispatch_number,date,vehicles,trips1),
    )
    conn.commit()
    conn.close()
    return "New row inserted successfully"

def read_query():
    """Query the database for the top 20 rows of the Ubertrips table"""
    conn = sqlite3.connect("Ubertrips.db")
    cursor = conn.cursor()
    cursor.execute("SELECT * FROM Ubertrips LIMIT 20")
    print("Top 20 rows of the Ubertrips table")
    rows=cursor.fetchall()
    for row in rows:
        print(row)
    conn.close()
    print("Success")

def update_query(active_vehicles,trips, dispatching_base_number,date):
    """Update the records included within the Ubertrips database"""
    conn = sqlite3.connect("Ubertrips.db")
    cursor = conn.cursor()
    cursor.execute("UPDATE Ubertrips SET active_vehicles=?, trips=? \
    WHERE dispatching_base_number = ? AND date = ?" , \
    (active_vehicles, trips, dispatching_base_number, date,))
    print("Data Update")
    conn.commit()
    conn.close()
    print("Updated successfully")

def delete_query(dispatching_base_number,date):
    """Delete the record containing the provided dispatching base number and date"""
    conn = sqlite3.connect("Ubertrips.db")
    cursor = conn.cursor()
    cursor.execute("SELECT * FROM Ubertrips \
    WHERE (dispatching_base_number=? AND date=?)", (dispatching_base_number,date))
    deleted = cursor.fetchall()
    cursor.execute("DELETE FROM Ubertrips \
    WHERE (dispatching_base_number=? AND date=?)", (dispatching_base_number,date))
    conn.commit()
    print("Deleting data:")
    print(deleted)
    conn.close()
    print("Deleted successfully")

if __name__ == "__main__":
    create_query(test_dispatch,test_date,test_active,test_trips)
    read_query()
    update_query()
    delete_query()
