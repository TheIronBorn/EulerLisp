(def nil '())

(defn ffst (lst) (fst (fst lst)))
(defn frst (lst) (fst (rst lst)))
(defn rfst (lst) (rst (fst lst)))
(defn rrst (lst) (rst (rst lst)))

(defn fffst (lst) (fst (fst (fst lst))))
(defn ffrst (lst) (fst (fst (rst lst))))
(defn frfst (lst) (fst (rst (fst lst))))
(defn frrst (lst) (fst (rst (rst lst))))
(defn rffst (lst) (rst (fst (fst lst))))
(defn rfrst (lst) (rst (fst (rst lst))))
(defn rrfst (lst) (rst (rst (fst lst))))
(defn rrrst (lst) (rst (rst (rst lst))))

(defn ffffst (lst) (fst (fst (fst (fst lst)))))
(defn fffrst (lst) (fst (fst (fst (rst lst)))))
(defn ffrfst (lst) (fst (fst (rst (fst lst)))))
(defn ffrrst (lst) (fst (fst (rst (rst lst)))))
(defn frffst (lst) (fst (rst (fst (fst lst)))))
(defn frfrst (lst) (fst (rst (fst (rst lst)))))
(defn frrfst (lst) (fst (rst (rst (fst lst)))))
(defn frrrst (lst) (fst (rst (rst (rst lst)))))
(defn rfffst (lst) (rst (fst (fst (fst lst)))))
(defn rffrst (lst) (rst (fst (fst (rst lst)))))
(defn rfrfst (lst) (rst (fst (rst (fst lst)))))
(defn rfrrst (lst) (rst (fst (rst (rst lst)))))
(defn rrffst (lst) (rst (rst (fst (fst lst)))))
(defn rrfrst (lst) (rst (rst (fst (rst lst)))))
(defn rrrfst (lst) (rst (rst (rst (fst lst)))))
(defn rrrrst (lst) (rst (rst (rst (rst lst)))))

(defn range_ (from to step acc)
      (if (< to from) acc
          (range_ from (- to step) step (cons to acc))))
(defn range (from to (step 1))
      (range_ from to step '()))

; (defn flatmap (f arr)
;       (if (nil? arr)
;           '()
;           (append (f (fst arr))
;                 (flatmap f (rst arr)))))

(defn delete (elem lst)
  (if (= (fst lst) elem)
      (rst lst)
      (cons (fst lst)
            (delete elem (rst lst)))))

(defn delete-nth (n lst)
  (if (= n 0)
      (rst lst)
      (cons (fst lst)
            (delete-nth (dec n) (rst lst)))))

(defn select (pred arr)
  (~> arr
      (reduce &(if (pred &1) (cons &1 &2) &2) '())
      reverse))

(defn reject (pred arr)
  (~> arr
      (reduce &(unless (pred &1) (cons &1 &2) &2) '())
      reverse))

(defn reduce-sum (f arr)
  (reduce (fn (x acc) (+ acc (f x))) 0 arr))

(defn reduce-product (f arr)
  (reduce (fn (x acc) (* acc (f x))) 1 arr))

(defn reduce-max (f init arr)
  (reduce (fn (x acc) (max (f x) acc)) init arr))
(defn reduce-min (f init arr)
  (reduce (fn (x acc) (min (f x) acc)) init arr))

(defn reduce-max~ (f init arr)
  (reduce~ (fn (x acc) (max (f x) acc)) init arr))
(defn reduce-min~ (f init arr)
  (reduce~ (fn (x acc) (min (f x) acc)) init arr))

(defn max-by (f arr)
  (if (nil? arr)
      '()
      (fst
        (reduce
          (fn (x acc)
              (let ([res (f x)])
                (if (> res (rst acc))
                    (cons x res)
                    acc)))
            (cons (fst arr) (f (fst arr)))
            (rst arr)))))

(defn min-by (f arr)
  (if (nil? arr)
      '()
      (fst
        (reduce
          (fn (x acc)
              (let ([res (f x)])
                (if (< res (rst acc))
                    (cons x res)
                    acc)))
            (cons (fst arr) (f (fst arr)))
            (rst arr)))))

(defn all? (pred arr)
  (cond
    [(nil? arr) #t]
    [(pred (fst arr)) (all? pred (rst arr))]
    [else #f]))

(defn zip (. lists)
      (if (any? nil? lists)
          '()
          (cons (map fst lists)
                (apply zip (map rst lists)))))

(defn product (lst) (reduce * 1 lst))
(defn sum (lst) (reduce + 0 lst))

(defn product~ (lst) (reduce~ * 1 lst))
(defn sum~ (lst) (reduce~ + 0 lst))

(defn take (n lst)
      (if (or (zero? n) (nil? lst))
          '()
          (cons (fst lst) (take (dec n) (rst lst)))))

(defn chunks (size lst (acc '()))
      (if (< (length lst) size)
          acc
          (chunks size
                  (rst lst)
                  (push acc (take size lst)))))

(defn first~ (stream) (nth~ 0 stream))

(defn palindromic? (lst) (= lst (reverse lst)))

(defn last (arr)
  (if (nil? (rst arr))
      (fst arr)
      (last (rst arr))))

(defn flatten (lst) (flatmap (fn (x) x) lst))

(defn each (f lst)
  (reduce (fn (cur acc) (f cur)) '() lst))

(defn empty? (lst) {(length lst) = 0})