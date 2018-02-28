; Solved: 28.2.2018

(def blocks
  (vector (list  0  1  2  9 10 11 18 19 20)
          (list  3  4  5 12 13 14 21 22 23)
          (list  6  7  8 15 16 17 24 25 26)
          (list 27 28 29 36 37 38 45 46 47)
          (list 30 31 32 39 40 41 48 49 50)
          (list 33 34 35 42 43 44 51 52 53)
          (list 54 55 56 63 64 65 72 73 74)
          (list 57 58 59 66 67 68 75 76 77)
          (list 60 61 62 69 70 71 78 79 80)))

(def rows
  (vector (list  0  1  2  3  4  5  6  7  8)
          (list  9 10 11 12 13 14 15 16 17)
          (list 18 19 20 21 22 23 24 25 26)
          (list 27 28 29 30 31 32 33 34 35)
          (list 36 37 38 39 40 41 42 43 44)
          (list 45 46 47 48 49 50 51 52 53)
          (list 54 55 56 57 58 59 60 61 62)
          (list 63 64 65 66 67 68 69 70 71)
          (list 72 73 74 75 76 77 78 79 80)))

(def cols
  (vector (list 0  9 18 27 36 45 54 63 72)
          (list 1 10 19 28 37 46 55 64 73)
          (list 2 11 20 29 38 47 56 65 74)
          (list 3 12 21 30 39 48 57 66 75)
          (list 4 13 22 31 40 49 58 67 76)
          (list 5 14 23 32 41 50 59 68 77)
          (list 6 15 24 33 42 51 60 69 78)
          (list 7 16 25 34 43 52 61 70 79)
          (list 8 17 26 35 44 53 62 71 80)))

(def neighbours
  (~> (range~ 0 80)
      (map~ (fn (idx)
              (let* ([ridx (div idx 9)]
                     [cidx (% idx 9)]
                     [bidx (+ (* (div ridx 3) 3) (div cidx 3))])
                (append
                  (vector-ref rows ridx)
                  (append
                    (vector-ref cols cidx)
                    (vector-ref blocks bidx))))))
      collect
      list->vector))

;; Get the indices for the row, column and block
;; a given position is in

(defn indices (pos)
  (let* ([ridx (div pos 9)]
         [cidx (% pos 9)]
         [bidx (+ (* (div ridx 3) 3)
                  (div cidx 3))])
    (list ridx cidx bidx)))

(def sudokus
  (~> (input-file-lines "project_euler/input-files/96.txt")
      (slices 10)
      (map &(~> &1 rst
               (apply str)
               string->chars
               (map char->digit)
               list->vector))))

;; Find all values that would fit into a field

(defn candidates (s idx)
  (let ([b (make-vector 10 #t)])
    (each &(vector-set! b (vector-ref s &1) #f)
          (vector-ref neighbours idx))
    (~> (range 1 9)
        (select &(vector-ref b &1)))))

;; Collect the indices of all free fields
;; together with their possible values.

(defn empty-fields (s)
  (~> (range~ 0 80)
      (select~ &(zero? (vector-ref s &1)))
      (map~ (fn (pos)
              (list pos
                    (indices pos)
                    (candidates s pos))))
      collect))

(defn partition (pred l (c1 '()) (c2 '()))
  (cond
    [(nil? l) (cons c1 c2)]
    [(pred (fst l))
     (partition pred (rst l)
                (cons (fst l) c1)
                c2)]
    [else
     (partition pred (rst l)
                c1
                (cons (fst l) c2))]))

(defn shares-idx? (a b)
  (or (= (fst a) (fst b))
      (= (frst a) (frst b))
      (= (frrst a) (frrst b))))

(defn update-neighbours (neighs unit)
  (map
    (fn (n)
      (if (shares-idx? (frst n) (frst unit))
          (list
            (fst n)
            (frst n)
            (delete (ffrrst unit) (frrst n)))
          n))
    neighs))

(defn update-all-neighbours (neighs units)
  (if (nil? units)
      neighs
      (update-all-neighbours
        (update-neighbours neighs (fst units))
        (rst units))))

(defn phase1 (s candidates)
  (let ([c (partition &(= 1 (length (frrst &1))) candidates)])
    (if (nil? (fst c))
        (map
          &(list
             (fst &1)
             (frst &1)
             (list->vector (frrst &1)))
          candidates)
        (do
          (each &(vector-set! s (fst &1) (ffrrst &1))
                (fst c))
          (phase1 s (update-all-neighbours (rst c) (fst c)))))))

(defn valid? (idx val rest)
  (none? &(= val (vector-ref rest &1))
         (vector-ref neighbours idx)))

(defn phase2 (s free idx)
  (if {idx >= (vector-length free)}
      s
      (let ([f (vector-ref free idx)])
        (~> (vector~ (frrst f))
            (select~ &(valid? (fst f) &1 s))
            (map~ &(let ([w (vector-copy s)])
                        (vector-set! w (fst f) &1)
                        w))
            (map~ &(phase2 &1 free (inc idx)))
            (select~ &(not (nil? &1)))
            first~))))

(defn solve (s)
  (let ([remaining (phase1 s (empty-fields s))])
    (if (nil? remaining)
        s
        (phase2 s (list->vector remaining) 0))))

(defn value (s)
  (+ (* 100 (vector-ref s 0))
     (*  10 (vector-ref s 1))
     (vector-ref s 2)))

(~> sudokus
    (reduce-sum &(~> &1 solve value))
    solution)
