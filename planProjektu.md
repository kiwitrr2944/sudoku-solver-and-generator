Sudoku Solver + Generator

Chciałbym stworzyć aplikację, która umożliwi użytkownikowi generowanie sudoku o wzbogaconych regułach.
Użytkownik wybiera reguły i wstawia część cyfr (może zero), a następnie program znajduje mu początkową planszę, która ma jednoznaczne rozwiązanie

Przykładowe dodatkowe zasady:
- podzbiór pól sumuje się do zdefiniowanej przez twórcę wartości
- na przekątnych znaleźć ma się permutacja od 1 do n
- zamiast klasycznych kwadratów można zdefinować dowolne obszary o polu n, które mają zawierać permutację
- więcej pomysłów może pojawić się w trakcie

Dodatkowym feature aplikacji będą:
- możliwość rozwiązywania wygenerowanego sudoku przez użytkownika
- wyświetlanie "podpowiedzi", czyli wszystkich możliwych, niesprzecznych liczb do wpisania (coś jakby silnik rozwiązujący z głębokością 1)
- kolorowy interfejs graficzny
* możliwość zapisania wygenerowanego sudoku z customowymi zasadami na później

W generowaniu planszy można będzie pokusić się o jakąś współbieżność przy backtrackowaniu.
Ogólnie efektywność programu będzie ważnym elementem całej aplikacji, na razie założyłbym, że celuję w program generujący sudoko 6x6 z przyczyn obliczeniowych, jeśli się uda to chciałbym to doprowadzić do 9x9

Etap 1:
- napisanie programu, który dla wejścia będącego standardową planszą sudoku znajduje jego rozwiązanie (rekurencyjnie i jednowątkowo)
- stworzenie interfejsu, który pozwala dodawać dodatkowe ograniczenia, przykładowo zbiory pól, które mają spełniać jeden z dwóch warunków (suma/bycie permutacją)
- wzbogacenie programu rozwiązującego o sprawdzanie dodanych powyższych warunków
- powyższe rzeczy na razie będą działy się w konsoli i miały CLI 

Etap 2:
- dodanie interfejsu graficznego do rozwiązywania
- dodanie interfejsu graficznego do generowania 
- stworzenie formatu pliku, możliwość zapisywania/wgrywania swoich plansz (wcześniej użytkownik będzie musiał sobie przerysowywać)
* uwspółbieżnienie przeszukiwania

Wiktor Rutecki