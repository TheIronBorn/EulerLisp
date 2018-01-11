; Solved 1.1

(defn leap-year? (n)
  (cond
    (divides? 400 n) #t
    (divides? 100 n) #f
    (divides? 4 n) #t
    else #f))

(defn month-days (month year)
  (case month
    (1 31)
    (2 (if (leap-year? year) 29 28))
    (3 31)
    (4 30)
    (5 31)
    (6 30)
    (7 31)
    (8 31)
    (9 30)
    (10 31)
    (11 30)
    (12 31)))

(defn next-date (date)
    (let (day (fst date)
          month (frst date)
          year (frrst date))
      (if (= day (month-days month year))
          (if (= month 12)
            (list 1 1 (inc year))
            (list 1 (inc month) year))
          (list (inc day) month year))))

(defn step (from to wday acc)
      (if (= from to)
          acc
          (let (day (fst from)
                month (frst from)
                year (frrst from))
               (step
                 (next-date from)
                 to
                 (% (inc wday) 7)
                 (if (and (> year 1900) (<= year 2000)
                          (= day 1)
                          (= wday 6))
                     (inc acc)
                     acc
                )))))

(println
  "Solution: "
  (step (list 1 1 1900)
        (list 1 1 2001)
        0
        0))
