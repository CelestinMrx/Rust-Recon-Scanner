# Le projet : "Scanner de reconnaissance réseau" (mini-Nmap + vulnérabilités basiques)

Idée : un CLI qui fait la phase de reconnaissance qu'un pentester ferait en début de mission — découverte d'hôtes, scan de ports, fingerprinting de services.

Fonctionnalités par palier

## Palier 1 — Scan de base (3-4 jours)

- Scan de ports TCP concurrent (avec tokio pour la rapidité — async Rust, très recherché)
- Détection host up/down (ICMP ou TCP connect)
- Bannière grabbing basique (récupérer ce que le service renvoie sur connexion)

## Palier 2 — Fingerprinting (4-5 jours)

- Détection de service/version à partir des bannières (HTTP, SSH, FTP, SMTP...)
- Scan de plages d'IP / CIDR en parallèle
- Détection de ports "intéressants" (ceux souvent mal configurés : 21, 22, 445, 3389, 8080...)

## Palier 3 — Aller plus loin

- Petit module de vérification de vulnérabilités connues (ex: comparer une version de service à une liste CVE basique)
- Export JSON/HTML du rapport
- Rate limiting / mode "stealth" pour montrer que tu comprends l'aspect discrétion

## Stack technique
- `tokio` — concurrence asynchrone (compétence très valorisée, même hors sécu)
- `pnet` ou raw sockets — manipulation de paquets bas niveau
- `clap` — CLI
- `serde_json` — export de rapport




Madame, Monsieur,

Actuellement étudiant en dernière année de Master Cybersécurité à la CyberSchool de Rennes, je recherche un stage de fin d’études de 6 mois, disponible immédiatement pour mettre en pratique mes compétences en cybersécurité au sein de votre entreprise.

Lors de mon stage à l’ANSSI, j’ai conduit une étude comparative approfondie d’outils SAST (Coverity, Klocwork, SonarQube). Afin de garantir l’objectivité et la reproductibilité de mes tests, j’ai développé des scripts d’automatisation en Python pour exécuter systématiquement les analyses sur un même jeu de données, en extraire les métriques clés et générer des rapports comparatifs. Cette approche méthodique a permis d’évaluer précisément les performances de chaque solution et a aidé les équipes de l’ANSSI à réduire de près de 60 % le taux de faux positifs.

Parallèlement, j’ai consolidé mes compétences en cybersécurité offensive via des plateformes comme Root-Me (3500+ points) et HackTheBox (dizaines de labs réalisés). J’ai notamment validé près de 50 % des challenges Web et Pentest, en réalisant par exemple des contournements de jetons JWT ainsi que l’exploitation de failles CSRF et XSS ainsi que des CVE récentes sur HackTheBox tels que React2Shell. Cette pratique concrète m’a permis de maı̂triser les vulnérabilités de l’OWASP Top 10 et les méthodologies de pentest web. Enfin, j’assure une veille technologique et sécurité constante via des médias spécialisés comme IT-Connect et Korben.info.

Je développe actuellement, à titre personnel, un scanner de reconnaissance réseau avec détection de vulnérabilités en Rust, un langage que je souhaite maı̂triser davantage. L’objectif est de concevoir un outil inspiré de Nmap, plus discret et adapté à mes besoins, que je pourrai notamment utiliser sur des plateformes d’entraı̂nement comme HackTheBox. Je prévois par la suite d’étendre ce projet vers un analyseur statique de malware, toujours en Rust.

Rejoindre Orasys représente pour moi une opportunité idéale pour approfondir mes compétences en tests d’intrusion au sein d’une équipe spécialisée. Les sujets proposés en exemple, tels que ≪ Le Dark Web au service de l’OSINT ≫ ou le développement et l’amélioration d’outils de pentest, m’intéressent particulièrement et font écho à mes compétences en développement et en sécurité offensive. Je suis convaincu que ce stage constituerait une étape clé pour consolider mon expertise technique tout en découvrant les exigences méthodologiques et rédactionnelles propres au métier de pentester.

Je me tiens à votre disposition pour tout renseignement complémentaire et serais ravi d’échanger avec vous lors d’un entretien.

Dans l’attente de votre retour, je vous prie d’agréer, Madame, Monsieur, l’expression de mes salutations distinguées.

MIREUX Célestin