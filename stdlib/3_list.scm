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

(defn length (lst)
  (defn inner (lst acc)
    (if (nil? lst)
        acc
        (inner (rst lst) (inc acc))))
  (inner lst 0))

(defn nth (n lst)
  (if (zero? n)
      (fst lst)
      (nth (dec n) (rst lst))))

(defn reverse (lst)
  (defn inner (lst acc)
    (if (nil? lst)
        acc
        (inner (rst lst) (cons (fst lst) acc))))
  (inner lst '()))

(defn range_ (from to step acc)
      (if (< to from) acc
          (range_ from (- to step) step (cons to acc))))
(defn range (from to . r)
  (if (nil? r)
      (range_ from to 1 '())
      (range_ from to (fst r) '())))

(defn any? (pred lst)
  (cond
    [(nil? lst) #f]
    [(pred (fst lst)) #t]
    [else (any? pred (rst lst))]))

(defn all? (pred lst)
  (cond
    [(nil? lst) #t]
    [(pred (fst lst)) (all? pred (rst lst))]
    [else #f]))

(defn count (pred lst)
  (defn inner (lst acc)
    (if (nil? lst)
        acc
        (inner (rst lst)
               (if (pred (fst lst)) (inc acc) acc))))
  (inner lst 0))

(defn map (f lst)
  (defn inner (lst acc)
    (if (nil? lst)
        (reverse acc)
        (inner  (rst lst) (cons (f (fst lst)) acc))))
  (inner lst '()))

(defn map* (f lsts)
  (defn inner (lsts acc)
    (if (any? nil? lsts)
        (reverse acc)
        (inner (map rst lsts)
               (cons (apply f (map fst lsts))
                     acc))))
  (inner lsts '()))

(defn transpose (lsts) (map* list lsts))

(defn reduce (f acc lst)
  (if (nil? lst)
      acc
      (reduce f (f (fst lst) acc) (rst lst))))

(defn append_ (a b)
  (if (nil? a)
      b (append_ (rst a) (cons (fst a) b))))
(defn append (a b) (append_ (reverse a) b))

; TODO: TCO
(defn flatmap (f lst)
      (if (nil? lst)
          '()
          (append (f (fst lst))
                  (flatmap f (rst lst)))))

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

(defn select (pred lst)
  (~> lst
      (reduce &(if (pred &1) (cons &1 &2) &2) '())
      reverse))

(defn reject (pred lst)
  (~> lst
      (reduce &(unless (pred &1) (cons &1 &2) &2) '())
      reverse))

(defn reduce-sum (f lst)
  (reduce (fn (x acc) (+ acc (f x))) 0 lst))

(defn reduce-product (f lst)
  (reduce (fn (x acc) (* acc (f x))) 1 lst))

(defn reduce-max (f init lst)
  (reduce (fn (x acc) (max (f x) acc)) init lst))
(defn reduce-min (f init lst)
  (reduce (fn (x acc) (min (f x) acc)) init lst))

(defn reduce-max~ (f init lst)
  (reduce~ (fn (x acc) (max (f x) acc)) init lst))
(defn reduce-min~ (f init lst)
  (reduce~ (fn (x acc) (min (f x) acc)) init lst))

(defn max-by (f lst)
  (if (nil? lst)
      '()
      (fst
        (reduce
          (fn (x acc)
              (let ([res (f x)])
                (if (> res (rst acc))
                    (cons x res)
                    acc)))
            (cons (fst lst) (f (fst lst)))
            (rst lst)))))

(defn min-by (f lst)
  (if (nil? lst)
      '()
      (fst
        (reduce
          (fn (x acc)
              (let ([res (f x)])
                (if (< res (rst acc))
                    (cons x res)
                    acc)))
            (cons (fst lst) (f (fst lst)))
            (rst lst)))))

(defn list-product (lst) (reduce * 1 lst))
(defn list-sum (lst) (reduce + 0 lst))
(defn list-min (lst) 
  (if (nil? lst)
      '()
      (reduce min (fst lst) (rst lst))))
(defn list-max (lst) 
  (if (nil? lst)
      '()
      (reduce max (fst lst) (rst lst))))

(defn product~ (lst) (reduce~ * 1 lst))
(defn sum~ (lst) (reduce~ + 0 lst))

(defn take (n lst)
  (defn inner (n lst acc)
    (if (or (zero? n) (nil? lst))
      (reverse acc)
      (inner (dec n) (rst lst)
            (cons (fst lst) acc))))
  (inner n lst '()))

(defn chunks (size lst)
  (defn inner (lst acc)
    (if (< (length lst) size)
        (reverse acc)
        (inner (rst lst)
               (cons (take size lst) acc))))
  (inner lst '()))

(defn first~ (stream) (nth~ 0 stream))

(defn palindromic? (lst) (= lst (reverse lst)))

(defn last (lst)
  (if (nil? (rst lst))
      (fst lst)
      (last (rst lst))))

(defn flatten (lst) (flatmap (fn (x) x) lst))

(defn each (f lst)
  (reduce (fn (cur acc) (f cur)) '() lst))

(defn each~ (f lst)
  (reduce~ (fn (cur acc) (f cur)) '() lst))

(defn empty? (lst) {(length lst) = 0})
