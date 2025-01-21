--
-- PostgreSQL database dump
--

-- Dumped from database version 17.2
-- Dumped by pg_dump version 17.2

SET statement_timeout = 0;
SET lock_timeout = 0;
SET idle_in_transaction_session_timeout = 0;
SET transaction_timeout = 0;
SET client_encoding = 'UTF8';
SET standard_conforming_strings = on;
SELECT pg_catalog.set_config('search_path', '', false);
SET check_function_bodies = false;
SET xmloption = content;
SET client_min_messages = warning;
SET row_security = off;

--
-- Name: type_build; Type: TYPE; Schema: public; Owner: smithy
--

CREATE TYPE public.type_build AS ENUM (
    'ANT',
    'BRD',
    'CPU',
    'DSK'
);


ALTER TYPE public.type_build OWNER TO smithy;

--
-- Name: type_rarity; Type: TYPE; Schema: public; Owner: smithy
--

CREATE TYPE public.type_rarity AS ENUM (
    'Common',
    'Uncommon',
    'Rare',
    'Legendary'
);


ALTER TYPE public.type_rarity OWNER TO smithy;

--
-- Name: type_cardslot; Type: TYPE; Schema: public; Owner: smithy
--

CREATE TYPE public.type_cardslot AS (
	set integer,
	rarity public.type_rarity,
	number integer
);


ALTER TYPE public.type_cardslot OWNER TO smithy;

--
-- Name: type_detail; Type: TYPE; Schema: public; Owner: smithy
--

CREATE TYPE public.type_detail AS ENUM (
    'Institution',
    'Role',
    'Location',
    'Distro'
);


ALTER TYPE public.type_detail OWNER TO smithy;

--
-- Name: type_host; Type: TYPE; Schema: public; Owner: smithy
--

CREATE TYPE public.type_host AS ENUM (
    'None',
    'Local',
    'Remote'
);


ALTER TYPE public.type_host OWNER TO smithy;

--
-- Name: type_kind; Type: TYPE; Schema: public; Owner: smithy
--

CREATE TYPE public.type_kind AS ENUM (
    'Analyze',
    'Breach',
    'Compute',
    'Disrupt'
);


ALTER TYPE public.type_kind OWNER TO smithy;

--
-- Name: type_missionlinkstate; Type: TYPE; Schema: public; Owner: smithy
--

CREATE TYPE public.type_missionlinkstate AS ENUM (
    'Closed',
    'Open'
);


ALTER TYPE public.type_missionlinkstate OWNER TO smithy;

--
-- Name: type_missionlink; Type: TYPE; Schema: public; Owner: smithy
--

CREATE TYPE public.type_missionlink AS (
	target integer,
	state public.type_missionlinkstate
);


ALTER TYPE public.type_missionlink OWNER TO smithy;

--
-- Name: type_missionnode; Type: TYPE; Schema: public; Owner: smithy
--

CREATE TYPE public.type_missionnode AS ENUM (
    'AccessPoint',
    'Backend',
    'Control',
    'Database',
    'Engine',
    'Frontend',
    'Gateway',
    'Hardware'
);


ALTER TYPE public.type_missionnode OWNER TO smithy;

--
-- Name: type_missionnodestate; Type: TYPE; Schema: public; Owner: smithy
--

CREATE TYPE public.type_missionnodestate AS ENUM (
    'Unknown',
    'Known'
);


ALTER TYPE public.type_missionnodestate OWNER TO smithy;

SET default_tablespace = '';

SET default_table_access_method = heap;

--
-- Name: build; Type: TABLE; Schema: public; Owner: smithy
--

CREATE TABLE public.build (
    id integer NOT NULL,
    kind public.type_build NOT NULL,
    company integer NOT NULL,
    market integer NOT NULL,
    number integer NOT NULL,
    title text,
    cardslot_1 public.type_cardslot NOT NULL,
    cardslot_2 public.type_cardslot NOT NULL,
    cardslot_3 public.type_cardslot NOT NULL,
    cardslot_4 public.type_cardslot NOT NULL,
    cardslot_5 public.type_cardslot NOT NULL,
    cardslot_6 public.type_cardslot NOT NULL,
    cardslot_7 public.type_cardslot NOT NULL,
    cardslot_8 public.type_cardslot NOT NULL,
    cardslot_9 public.type_cardslot NOT NULL,
    cardslot_10 public.type_cardslot NOT NULL,
    cardslot_11 public.type_cardslot NOT NULL,
    cardslot_12 public.type_cardslot NOT NULL,
    cardslot_13 public.type_cardslot NOT NULL,
    cardslot_14 public.type_cardslot NOT NULL,
    cardslot_15 public.type_cardslot NOT NULL
);


ALTER TABLE public.build OWNER TO smithy;

--
-- Name: build/company; Type: TABLE; Schema: public; Owner: smithy
--

CREATE TABLE public."build/company" (
    id integer NOT NULL,
    name text,
    kind public.type_build NOT NULL
);


ALTER TABLE public."build/company" OWNER TO smithy;

--
-- Name: build/market; Type: TABLE; Schema: public; Owner: smithy
--

CREATE TABLE public."build/market" (
    id integer NOT NULL,
    name text NOT NULL
);


ALTER TABLE public."build/market" OWNER TO smithy;

--
-- Name: build/meta_2_id_seq; Type: SEQUENCE; Schema: public; Owner: smithy
--

CREATE SEQUENCE public."build/meta_2_id_seq"
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE public."build/meta_2_id_seq" OWNER TO smithy;

--
-- Name: build/meta_2_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: smithy
--

ALTER SEQUENCE public."build/meta_2_id_seq" OWNED BY public."build/company".id;


--
-- Name: build/meta_3_id_seq; Type: SEQUENCE; Schema: public; Owner: smithy
--

CREATE SEQUENCE public."build/meta_3_id_seq"
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE public."build/meta_3_id_seq" OWNER TO smithy;

--
-- Name: build/meta_3_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: smithy
--

ALTER SEQUENCE public."build/meta_3_id_seq" OWNED BY public."build/market".id;


--
-- Name: build_id_seq; Type: SEQUENCE; Schema: public; Owner: smithy
--

ALTER TABLE public.build ALTER COLUMN id ADD GENERATED BY DEFAULT AS IDENTITY (
    SEQUENCE NAME public.build_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1
);


--
-- Name: card; Type: TABLE; Schema: public; Owner: smithy
--

CREATE TABLE public.card (
    id integer NOT NULL,
    title text NOT NULL,
    rarity public.type_rarity NOT NULL,
    number integer NOT NULL,
    set integer NOT NULL,
    kind public.type_kind NOT NULL,
    cost integer NOT NULL,
    delay integer NOT NULL,
    priority integer NOT NULL,
    rules_launch text NOT NULL,
    rules_run text NOT NULL,
    host public.type_host
);


ALTER TABLE public.card OWNER TO smithy;

--
-- Name: card_id_seq; Type: SEQUENCE; Schema: public; Owner: smithy
--

ALTER TABLE public.card ALTER COLUMN id ADD GENERATED BY DEFAULT AS IDENTITY (
    SEQUENCE NAME public.card_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1
);


--
-- Name: detail; Type: TABLE; Schema: public; Owner: smithy
--

CREATE TABLE public.detail (
    id integer NOT NULL,
    kind public.type_detail NOT NULL,
    general integer NOT NULL,
    specific integer NOT NULL,
    number integer NOT NULL,
    title text NOT NULL,
    cardslot_1 public.type_cardslot NOT NULL,
    cardslot_2 public.type_cardslot NOT NULL,
    cardslot_3 public.type_cardslot NOT NULL,
    cardslot_4 public.type_cardslot NOT NULL,
    cardslot_5 public.type_cardslot NOT NULL,
    cardslot_6 public.type_cardslot NOT NULL,
    cardslot_7 public.type_cardslot NOT NULL,
    cardslot_8 public.type_cardslot NOT NULL,
    cardslot_9 public.type_cardslot NOT NULL,
    cardslot_10 public.type_cardslot NOT NULL,
    cardslot_11 public.type_cardslot NOT NULL,
    cardslot_12 public.type_cardslot NOT NULL,
    cardslot_13 public.type_cardslot NOT NULL,
    cardslot_14 public.type_cardslot NOT NULL,
    cardslot_15 public.type_cardslot NOT NULL
);


ALTER TABLE public.detail OWNER TO smithy;

--
-- Name: category_id_seq; Type: SEQUENCE; Schema: public; Owner: smithy
--

ALTER TABLE public.detail ALTER COLUMN id ADD GENERATED BY DEFAULT AS IDENTITY (
    SEQUENCE NAME public.category_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1
);


--
-- Name: detail/general; Type: TABLE; Schema: public; Owner: smithy
--

CREATE TABLE public."detail/general" (
    id integer NOT NULL,
    name text,
    kind public.type_detail NOT NULL
);


ALTER TABLE public."detail/general" OWNER TO smithy;

--
-- Name: detail/meta_2_id_seq; Type: SEQUENCE; Schema: public; Owner: smithy
--

CREATE SEQUENCE public."detail/meta_2_id_seq"
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE public."detail/meta_2_id_seq" OWNER TO smithy;

--
-- Name: detail/meta_2_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: smithy
--

ALTER SEQUENCE public."detail/meta_2_id_seq" OWNED BY public."detail/general".id;


--
-- Name: detail/specific; Type: TABLE; Schema: public; Owner: smithy
--

CREATE TABLE public."detail/specific" (
    id integer NOT NULL,
    name text NOT NULL,
    general integer NOT NULL
);


ALTER TABLE public."detail/specific" OWNER TO smithy;

--
-- Name: detail/meta_3_id_seq; Type: SEQUENCE; Schema: public; Owner: smithy
--

CREATE SEQUENCE public."detail/meta_3_id_seq"
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE public."detail/meta_3_id_seq" OWNER TO smithy;

--
-- Name: detail/meta_3_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: smithy
--

ALTER SEQUENCE public."detail/meta_3_id_seq" OWNED BY public."detail/specific".id;


--
-- Name: mission; Type: TABLE; Schema: public; Owner: smithy
--

CREATE TABLE public.mission (
    id integer NOT NULL,
    mission_id integer NOT NULL,
    node_id integer NOT NULL,
    kind public.type_missionnode NOT NULL,
    state public.type_missionnodestate NOT NULL,
    north public.type_missionlink,
    east public.type_missionlink,
    south public.type_missionlink,
    west public.type_missionlink
);


ALTER TABLE public.mission OWNER TO smithy;

--
-- Name: mission_id_seq; Type: SEQUENCE; Schema: public; Owner: smithy
--

ALTER TABLE public.mission ALTER COLUMN id ADD GENERATED ALWAYS AS IDENTITY (
    SEQUENCE NAME public.mission_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1
);


--
-- Name: build/company id; Type: DEFAULT; Schema: public; Owner: smithy
--

ALTER TABLE ONLY public."build/company" ALTER COLUMN id SET DEFAULT nextval('public."build/meta_2_id_seq"'::regclass);


--
-- Name: build/market id; Type: DEFAULT; Schema: public; Owner: smithy
--

ALTER TABLE ONLY public."build/market" ALTER COLUMN id SET DEFAULT nextval('public."build/meta_3_id_seq"'::regclass);


--
-- Name: detail/general id; Type: DEFAULT; Schema: public; Owner: smithy
--

ALTER TABLE ONLY public."detail/general" ALTER COLUMN id SET DEFAULT nextval('public."detail/meta_2_id_seq"'::regclass);


--
-- Name: detail/specific id; Type: DEFAULT; Schema: public; Owner: smithy
--

ALTER TABLE ONLY public."detail/specific" ALTER COLUMN id SET DEFAULT nextval('public."detail/meta_3_id_seq"'::regclass);


--
-- Data for Name: build; Type: TABLE DATA; Schema: public; Owner: smithy
--

COPY public.build (id, kind, company, market, number, title, cardslot_1, cardslot_2, cardslot_3, cardslot_4, cardslot_5, cardslot_6, cardslot_7, cardslot_8, cardslot_9, cardslot_10, cardslot_11, cardslot_12, cardslot_13, cardslot_14, cardslot_15) FROM stdin;
211	DSK	14	1	3	ShareAll BX2A	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
36	ANT	3	1	4	MX1000 Red	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
80	BRD	5	4	4	I01-640-TMN59	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
70	BRD	5	2	2	X01-570-DBI87	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
9	ANT	1	3	1	Strata 330a	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
169	CPU	11	3	1	OpenServ 04	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
37	ANT	3	2	1	AX1500 Chocolate	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
178	CPU	12	1	2	Cap Madder	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
155	CPU	10	3	3	Newton	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
16	ANT	1	4	4	Mosaic 210o	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
69	BRD	5	2	1	X01-570-BYA65	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
71	BRD	5	2	3	X01-570-SBZ59	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
204	DSK	13	3	4	Black Gen4	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
240	DSK	15	4	4	SOHOSafe Extra	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
208	DSK	13	4	4	Green Gen4	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
87	BRD	6	2	3	C2 Cruise	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
163	CPU	11	1	3	OpenCore 4-X	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
141	CPU	9	4	1	CC3105	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
72	BRD	5	2	4	X01-570-VFI41	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
202	DSK	13	3	2	Black Gen2	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
65	BRD	5	1	1	A01-490-BGN22	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
107	BRD	7	3	3	Opus	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
10	ANT	1	3	2	Strata 330b	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
78	BRD	5	4	2	I01-640-OMA02	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
252	DSK	16	3	4	Depot 4000	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
45	ANT	3	4	1	LX2000 Barley	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
49	ANT	4	1	1	Albarino	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
162	CPU	11	1	2	OpenCore 2-Y	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
34	ANT	3	1	2	MX1000 Black	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
148	CPU	10	1	4	Leavitt	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
200	DSK	13	2	4	Yellow Gen4	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
94	BRD	6	4	2	S5 Silent	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
89	BRD	6	3	1	T7 Tantamount	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
114	BRD	8	1	2	Frequency H350H	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
33	ANT	3	1	1	MX1000 Amber	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
245	DSK	16	2	1	Silo 40	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
147	CPU	10	1	3	Galilei	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
228	DSK	15	1	4	HomeSafe Extra	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
105	BRD	7	3	1	Contribuens	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
238	DSK	15	4	2	SOHOSafe Medium	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
54	ANT	4	2	2	Garganega	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
29	ANT	2	4	1	ConnectNet Averna	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
81	BRD	6	1	1	L1 Leisure	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
73	BRD	5	3	1	S01-720-INT39	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
6	ANT	1	2	2	Amarillo 120b	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
19	ANT	2	1	3	CruiseNet Grappa	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
42	ANT	3	3	2	TX4000 Biscuit	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
43	ANT	3	3	3	TX4000 Chit	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
40	ANT	3	2	4	AX1500 Smoke	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
102	BRD	7	2	2	Concitator	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
251	DSK	16	3	3	Depot 3000	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
118	BRD	8	2	2	Conductance J460J	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
48	ANT	3	4	4	LX2000 Wheat	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
192	CPU	12	4	4	Peak Titanium	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
85	BRD	6	2	1	C2 Compete	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
223	DSK	14	4	3	AdvCollab PX3A	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
101	BRD	7	2	1	Adfici	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
111	BRD	7	4	3	Gratis	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
24	ANT	2	2	4	SurfNet Pastis	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
161	CPU	11	1	1	OpenCore 2-X	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
215	DSK	14	2	3	RapidShare BX5G	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
108	BRD	7	3	4	Professio	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
232	DSK	15	2	4	GameSafe Extra	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
5	ANT	1	2	1	Amarillo 120a	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
145	CPU	10	1	1	Brahe	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
28	ANT	2	3	4	XferNet Leopold	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
121	BRD	8	3	1	Cleave L660L	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
132	CPU	9	1	4	CC3300	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
222	DSK	14	4	2	AdvCollab PX1B	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
209	DSK	14	1	1	ShareAll BX1A	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
13	ANT	1	4	1	Mosaic 210a	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
142	CPU	9	4	2	CC3205	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
39	ANT	3	2	3	AX1500 Roast	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
15	ANT	1	4	3	Mosaic 210ab	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
8	ANT	1	2	4	Amarillo 120o	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
239	DSK	15	4	3	SOHOSafe Large	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
190	CPU	12	4	2	Peak Iron	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
58	ANT	4	3	2	Mourvedre	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
67	BRD	5	1	2	A01-490-RCZ62	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
44	ANT	3	3	4	TX4000 Pale	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
165	CPU	11	2	1	OpenCore 2-X+	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
138	CPU	9	3	2	CC3255	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
23	ANT	2	2	3	SurfNet Kir	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
172	CPU	11	3	4	OpenServ 16	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
247	DSK	16	2	3	Silo 80	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
113	BRD	8	1	1	Frequency H330H	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
173	CPU	11	4	1	OpenCore 6-X	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
93	BRD	6	4	1	S5 Signature	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
214	DSK	14	2	2	RapidShare BX3G	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
84	BRD	6	1	4	L1 Lucky	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
61	ANT	4	4	1	Cinsault	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
150	CPU	10	2	2	Herschel	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
41	ANT	3	3	1	TX4000 Acid	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
230	DSK	15	2	2	GameSafe Medium	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
110	BRD	7	4	2	Desideria	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
213	DSK	14	2	1	RapidShare BX1G	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
158	CPU	10	4	2	Eddington	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
220	DSK	14	3	4	ProCollab PX5D	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
50	ANT	4	1	2	Semillon	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
181	CPU	12	2	1	Mask Emerald	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
205	DSK	13	4	1	Green Gen1	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
136	CPU	9	2	4	CC3350	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
206	DSK	13	4	2	Green Gen2	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
195	DSK	13	1	3	Red Gen3	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
86	BRD	6	2	2	C2 Connect	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
99	BRD	7	1	3	Humilis	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
106	BRD	7	3	2	Negotium	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
62	ANT	4	4	2	Grenache	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
184	CPU	12	2	4	Mask Topaz	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
156	CPU	10	3	4	Sagan	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
112	BRD	7	4	4	Ingenuus	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
130	CPU	9	1	2	CC3100	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
25	ANT	2	3	1	XferNet Angelico	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
12	ANT	1	3	4	Strata 330o	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
104	BRD	7	2	4	Praeventus	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
14	ANT	1	4	2	Mosaic 210b	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
221	DSK	14	4	1	AdvCollab PX1A	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
194	DSK	13	1	2	Red Gen2	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
243	DSK	16	1	3	Stockpile 60	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
7	ANT	1	2	3	Amarillo 120ab	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
123	BRD	8	3	3	Fast L660L	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
229	DSK	15	2	1	GameSafe Small	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
82	BRD	6	1	2	L1 Lightweight	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
68	BRD	5	1	4	A01-490-WRE76	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
20	ANT	2	1	4	CruiseNet Tsikoudia	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
97	BRD	7	1	1	Beatus	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
143	CPU	9	4	3	CC3405	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
168	CPU	11	2	4	OpenCore 4-Y+	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
164	CPU	11	1	4	OpenCore 4-Y	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
66	BRD	5	1	3	A01-490-CRP35	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
225	DSK	15	1	1	HomeSafe Small	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
124	BRD	8	3	4	Sanction L660L	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
38	ANT	3	2	2	AX1500 Peat	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
207	DSK	13	4	3	Green Gen3	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
117	BRD	8	2	1	Conductance J440J	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
157	CPU	10	4	1	Airy	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
254	DSK	16	4	2	Repo 200	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
96	BRD	6	4	4	S5 Superior	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
144	CPU	9	4	4	CC3805	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
167	CPU	11	2	3	OpenCore 4-X+	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
197	DSK	13	2	1	Yellow Gen1	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
129	CPU	9	1	1	CC3000	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
64	ANT	4	4	4	Tempranillo	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
26	ANT	2	3	2	XferNet Branca	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
179	CPU	12	1	3	Cap Saffron	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
90	BRD	6	3	2	T7 Technical	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
255	DSK	16	4	3	Repo 300	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
183	CPU	12	2	3	Mask Sapphire	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
187	CPU	12	3	3	Bill Platinum	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
191	CPU	12	4	3	Peak Steel	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
134	CPU	9	2	2	CC3150	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
241	DSK	16	1	1	Stockpile 20	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
18	ANT	2	1	2	CruiseNet Chacha	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
177	CPU	12	1	1	Cap Indigo	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
128	BRD	8	4	4	Positional K550K	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
196	DSK	13	1	4	Red Gen4	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
77	BRD	5	4	1	I01-640-ECD71	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
55	ANT	4	2	3	Marsanne	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
131	CPU	9	1	3	CC3200	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
32	ANT	2	4	4	ConnectNet Nonino	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
234	DSK	15	3	2	WorkSafe Medium	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
116	BRD	8	1	4	Time H350H	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
95	BRD	6	4	3	S5 Special	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
244	DSK	16	1	4	Stockpile 80	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
115	BRD	8	1	3	Time H330H	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
135	CPU	9	2	3	CC3250	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
152	CPU	10	2	4	Rubin	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
91	BRD	6	3	3	T7 Total	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
151	CPU	10	2	3	Kepler	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
212	DSK	14	1	4	ShareAll BX2B	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
149	CPU	10	2	1	Cannon	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
76	BRD	5	3	4	S01-720-YFE31	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
154	CPU	10	3	2	Hubble	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
4	ANT	1	1	4	Citra 100o	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
46	ANT	3	4	2	LX2000 Oat	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
236	DSK	15	3	4	WorkSafe Extra	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
256	DSK	16	4	4	Repo 400	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
88	BRD	6	2	4	C2 Crunch	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
193	DSK	13	1	1	Red Gen1	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
56	ANT	4	2	4	Verdicchio	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
52	ANT	4	1	4	Viognier	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
160	CPU	10	4	4	Kuiper	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
119	BRD	8	2	3	Resistance J440J	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
249	DSK	16	3	1	Depot 1000	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
35	ANT	3	1	3	MX1000 Caramel	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
27	ANT	2	3	3	XferNet Francisco	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
189	CPU	12	4	1	Peak Carbon	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
133	CPU	9	2	1	CC3050	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
235	DSK	15	3	3	WorkSafe Large	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
233	DSK	15	3	1	WorkSafe Small	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
226	DSK	15	1	2	HomeSafe Medium	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
216	DSK	14	2	4	RapidShare BX7G	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
83	BRD	6	1	3	L1 LoFi	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
31	ANT	2	4	3	ConnectNet Montenegro	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
170	CPU	11	3	2	OpenServ 08	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
250	DSK	16	3	2	Depot 2000	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
219	DSK	14	3	3	ProCollab PX5C	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
74	BRD	5	3	2	S01-720-MJU09	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
159	CPU	10	4	3	Hale	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
2	ANT	1	1	2	Citra 100b	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
253	DSK	16	4	1	Repo 100	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
92	BRD	6	3	4	T7 Trust	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
21	ANT	2	2	1	SurfNet Arak	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
127	BRD	8	4	3	Optical K550K	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
175	CPU	11	4	3	OpenServ 01	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
203	DSK	13	3	3	Black Gen3	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
100	BRD	7	1	4	Laetissimus	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
201	DSK	13	3	1	Black Gen1	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
180	CPU	12	1	4	Cap Woad	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
174	CPU	11	4	2	OpenCore 6-Y	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
11	ANT	1	3	3	Strata 330ab	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
47	ANT	3	4	3	LX2000 Rye	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
246	DSK	16	2	2	Silo 60	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
79	BRD	5	4	3	I01-640-PFM38	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
139	CPU	9	3	3	CC3455	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
198	DSK	13	2	2	Yellow Gen2	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
125	BRD	8	4	1	Functional K550K	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
75	BRD	5	3	3	S01-720-UOA23	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
199	DSK	13	2	3	Yellow Gen3	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
22	ANT	2	2	2	SurfNet Chartreuse	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
103	BRD	7	2	3	Excitatur	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
122	BRD	8	3	2	Enjoin L660L	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
227	DSK	15	1	3	HomeSafe Large	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
231	DSK	15	2	3	GameSafe Large	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
166	CPU	11	2	2	OpenCore 2-Y+	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
188	CPU	12	3	4	Bill Silver	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
126	BRD	8	4	2	Geometric K550K	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
3	ANT	1	1	3	Citra 100ab	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
120	BRD	8	2	4	Resistance J460J	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
98	BRD	7	1	2	Felix	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
153	CPU	10	3	1	Einstein	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
137	CPU	9	3	1	CC3155	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
171	CPU	11	3	3	OpenServ 12	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
30	ANT	2	4	2	ConnectNet Del Capo	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
57	ANT	4	3	1	Carménère	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
63	ANT	4	4	3	Sangiovese	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
210	DSK	14	1	2	ShareAll BX1B	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
51	ANT	4	1	3	Torrontes	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
59	ANT	4	3	3	Nebbiolo	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
185	CPU	12	3	1	Bill Copper	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
53	ANT	4	2	1	Airen	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
17	ANT	2	1	1	CruiseNet Aquavit	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
217	DSK	14	3	1	ProCollab PX5A	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
248	DSK	16	2	4	Silo 100	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
242	DSK	16	1	2	Stockpile 40	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
182	CPU	12	2	2	Mask Ruby	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
60	ANT	4	3	4	Pinotage	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
176	CPU	11	4	4	OpenServ 02	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
224	DSK	14	4	4	AdvCollab PX3B	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
186	CPU	12	3	2	Bill Gold	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
140	CPU	9	3	4	CC3855	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
146	CPU	10	1	2	Copernicus	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
1	ANT	1	1	1	Citra 100a	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
237	DSK	15	4	1	SOHOSafe Small	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
218	DSK	14	3	2	ProCollab PX5B	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
109	BRD	7	4	1	Aspirare	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
\.


--
-- Data for Name: build/company; Type: TABLE DATA; Schema: public; Owner: smithy
--

COPY public."build/company" (id, name, kind) FROM stdin;
1	EXM	ANT
3	TransGlobal	ANT
7	SilPath	BRD
9	CC	CPU
5	Axis	BRD
10	Orbital	CPU
15	Vault	DSK
11	RiscFree	CPU
12	Visor	CPU
13	Evoke	DSK
4	Uplink	ANT
2	NetTECH	ANT
8	Wasbleibt	BRD
14	Kollectiv	DSK
16	Warehaus	DSK
6	PeriPeri	BRD
\.


--
-- Data for Name: build/market; Type: TABLE DATA; Schema: public; Owner: smithy
--

COPY public."build/market" (id, name) FROM stdin;
1	Consumer
2	Enthusiast
3	Professional
4	Prosumer
\.


--
-- Data for Name: card; Type: TABLE DATA; Schema: public; Owner: smithy
--

COPY public.card (id, title, rarity, number, set, kind, cost, delay, priority, rules_launch, rules_run, host) FROM stdin;
12	La Somnambule	Rare	4	1	Disrupt	1	1	99	TARG:MA|LOOP:05	DECV:OP:CC*DD|DECV:FS:05	Remote
1	The Hate	Common	1	1	Analyze	1	1	99	TARG:MA|LOOP:05	DECV:FS:05	Local
2	Le Reve	Common	2	1	Breach	1	1	99	TARG:MA|LOOP:05	DECV:TC:AA	Remote
15	Duality	Legendary	3	1	Compute	1	1	99	TARG:MA|LOOP:05	INCV:SH:BD|DECV:TC:AA	Local
3	Martyrium	Common	3	1	Compute	1	1	99	TARG:MA|LOOP:05	DECV:SH:BB+10	Local
13	Ad Infinitum	Legendary	1	1	Analyze	1	1	99	TARG:MA|LOOP:05	INCV:FS:10-AB|INCV:OP:CA	Local
8	Unreal Cinema	Uncommon	4	1	Disrupt	1	1	99	TARG:MA|LOOP:05	INCV:OP:CA	Remote
16	InHuman	Legendary	4	1	Disrupt	1	1	99	TARG:MA|LOOP:05	INCV:OP:CA|DECV:SH:BB+10	Remote
7	Hallucigenia	Uncommon	3	1	Compute	1	1	99	TARG:MA|LOOP:05	INCV:SH:BD	Local
11	Visitation	Rare	3	1	Compute	1	1	99	TARG:MA|LOOP:05	DECV:SH:BB+10|INCV:TC:CD	Local
5	Luminary	Uncommon	1	1	Analyze	1	1	99	TARG:MA|LOOP:05	INCV:FS:10-AB	Local
6	L'Âme Électrique	Uncommon	2	1	Breach	1	1	99	TARG:MA|LOOP:05	INCV:TC:CD	Remote
4	Machine Moderne	Common	4	1	Disrupt	1	1	99	TARG:MA|LOOP:05	DECV:OP:CC*DD	Remote
10	Phenomena	Rare	2	1	Breach	1	1	99	TARG:MA|LOOP:05	DECV:TC:AA|INCV:FS:10-AB	Remote
9	Rosa Aeterna	Rare	1	1	Analyze	1	1	99	TARG:MA|LOOP:05	DECV:FS:05|INCV:SH:BD	Local
14	Confessions	Legendary	2	1	Breach	1	1	99	TARG:MA|LOOP:05	INCV:TC:CD|DECV:OP:CC*DD	Remote
\.


--
-- Data for Name: detail; Type: TABLE DATA; Schema: public; Owner: smithy
--

COPY public.detail (id, kind, general, specific, number, title, cardslot_1, cardslot_2, cardslot_3, cardslot_4, cardslot_5, cardslot_6, cardslot_7, cardslot_8, cardslot_9, cardslot_10, cardslot_11, cardslot_12, cardslot_13, cardslot_14, cardslot_15) FROM stdin;
78	Institution	5	20	2	National Research Centre	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
65	Institution	5	17	1	Cairo University	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
256	Role	16	64	4	Plumber	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
105	Institution	7	27	1	Kamatayon GmbH	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
163	Location	11	41	3	Highrise	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
82	Institution	6	21	2	NITDA	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
45	Distro	3	12	1	Guardian	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
198	Role	13	50	2	Designer	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
193	Role	13	49	1	2D Art	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
77	Institution	5	20	1	KAIST	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
114	Institution	8	29	2	KeepTrack	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
54	Distro	4	14	2	Minotaur	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
132	Location	9	33	4	Parking Structure	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
162	Location	11	41	2	Duplex	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
13	Distro	1	4	1	Hyperspace	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
44	Distro	3	11	4	Obsidian	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
255	Role	16	64	3	Installer	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
252	Role	16	63	4	Procurement	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
230	Role	15	58	2	Coordinator	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
225	Role	15	57	1	Auditor	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
23	Distro	2	6	3	Silver Bullet	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
109	Institution	7	28	1	Olutalo	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
16	Distro	1	4	4	Supernova	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
22	Distro	2	6	2	Hawkeye	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
131	Location	9	33	3	Information Booth	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
130	Location	9	33	2	Company Store	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
133	Location	9	34	1	Coworking Space	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
161	Location	11	41	1	Condo	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
119	Institution	8	30	3	Overclox	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
223	Role	14	56	3	Helpdesk	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
195	Role	13	49	3	Audio	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
220	Role	14	55	4	Info Assurance	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
115	Institution	8	29	3	NetFree	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
112	Institution	7	28	4	Sota	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
134	Location	9	34	2	Open Floor	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
19	Distro	2	5	3	Jekyll	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
197	Role	13	50	1	Analyst	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
166	Location	11	42	2	Rural	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
73	Institution	5	19	1	EPFL	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
26	Distro	2	7	2	Griffin	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
50	Distro	4	13	2	Lost Horizon	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
108	Institution	7	27	4	SMRT Tech	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
113	Institution	8	29	1	Green Earth	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
194	Role	13	49	2	3D Art	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
58	Distro	4	15	2	Magellan	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
40	Distro	3	10	4	Quantum	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
160	Location	10	40	4	Tourist Area	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
129	Location	9	33	1	Cafeteria	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
251	Role	16	63	3	Logistics	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
159	Location	10	40	3	Library	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
190	Location	12	48	2	Derelict Building	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
253	Role	16	64	1	Electrician	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
227	Role	15	57	3	Payroll	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
91	Institution	6	23	3	NACS	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
8	Distro	1	2	4	Shimmer	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
192	Location	12	48	4	War Driving	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
106	Institution	7	27	2	Navi Systems	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
110	Institution	7	28	2	NTOA	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
168	Location	11	42	4	Urban	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
48	Distro	3	12	4	Vortex	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
231	Role	15	58	3	Facilities	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
229	Role	15	58	1	Clerk	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
81	Institution	6	21	1	GIS Egypt	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
87	Institution	6	22	3	NIST	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
52	Distro	4	13	4	Wonderland	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
164	Location	11	41	4	Townhouse	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
83	Institution	6	21	3	NRF | SAASTA	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
104	Institution	7	26	4	Ukoyisa Productions	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
139	Location	9	35	3	Private Room	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
156	Location	10	39	4	Taproom	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
31	Distro	2	8	3	Phoenix	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
120	Institution	8	30	4	The Society	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
155	Location	10	39	3	Restaurant	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
46	Distro	3	12	2	Hermitage	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
107	Institution	7	27	3	LiWA	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
15	Distro	1	4	3	Quasar	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
188	Location	12	47	4	POS Hole	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
95	Institution	6	24	3	NKP	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
88	Institution	6	22	4	Synco II	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
187	Location	12	47	3	Police Station	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
79	Institution	5	20	3	Sorbonne	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
75	Institution	5	19	3	Nanyang Technological	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
43	Distro	3	11	3	Majestic	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
199	Role	13	50	3	Producer	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
219	Role	14	55	3	Forensics	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
20	Distro	2	5	4	Soul	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
24	Distro	2	6	4	Zenith	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
123	Institution	8	31	3	GigHello	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
206	Role	13	52	2	Certification	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
203	Role	13	51	3	Fullstack	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
127	Institution	8	32	3	DistribuNet	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
76	Institution	5	19	4	Universite de Sfax	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
72	Institution	5	18	4	Zagazig University	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
222	Role	14	56	2	Desktop Support	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
136	Location	9	34	4	Single Unit	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
140	Location	9	35	4	Reception	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
215	Role	14	54	3	Repair	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
143	Location	9	36	3	Meeting Room	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
218	Role	14	55	2	Archivist	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
135	Location	9	34	3	Shared Unit	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
191	Location	12	48	3	Open Access Point	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
3	Distro	1	1	3	Hobbit	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
144	Location	9	36	4	Office	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
152	Location	10	38	4	Study Hall	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
175	Location	11	44	3	Hostel	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
51	Distro	4	13	3	Nirvana	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
27	Distro	2	7	3	Kraken	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
116	Institution	8	29	4	ReNu	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
202	Role	13	51	2	Frontend	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
111	Institution	7	28	3	Gubat	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
92	Institution	6	23	4	NLI	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
226	Role	15	57	2	Comptroller	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
183	Location	12	46	3	Network Closet	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
47	Distro	3	12	3	Terraforming	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
238	Role	15	60	2	Brand Management	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
234	Role	15	59	2	Business Partner	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
84	Institution	6	21	4	SITA	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
254	Role	16	64	2	HVAC	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
59	Distro	4	15	3	Oasis	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
68	Institution	5	17	4	TU Berlin	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
55	Distro	4	14	3	Mythos	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
250	Role	16	63	2	Inventory Control	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
167	Location	11	42	3	Suburban	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
171	Location	11	43	3	Lobby	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
7	Distro	1	2	3	Lucid	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
63	Distro	4	16	3	Harmony	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
35	Distro	3	9	3	Illusion	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
80	Institution	5	20	4	UC Berkeley	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
241	Role	16	61	1	Cleaning	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
214	Role	14	54	2	PC Build	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
236	Role	15	59	4	Relations	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
125	Institution	8	32	1	B-Ware	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
128	Institution	8	32	4	OpenSesame	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
6	Distro	1	2	2	Fantasia	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
148	Location	10	37	4	Shipping Center	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
66	Institution	5	17	3	Drexel	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
211	Role	14	53	3	Telemetry	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
209	Role	14	53	1	Database Admin	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
38	Distro	3	10	2	Lightspeed	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
93	Institution	6	24	1	CERT	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
180	Location	12	45	4	Traffic Control	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
207	Role	13	52	3	Dev Support	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
243	Role	16	61	3	Janitorial	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
39	Distro	3	10	3	Paradox	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
99	Institution	7	25	3	Olsgolon PLC	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
28	Distro	2	7	4	Skynet	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
29	Distro	2	8	1	Firebird	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
149	Location	10	38	1	Classroom	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
126	Institution	8	32	2	FOFA Foundation	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
150	Location	10	38	2	Commons	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
204	Role	13	51	4	Systems	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
246	Role	16	62	2	Event Staff	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
240	Role	15	60	4	Social Media	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
242	Role	16	61	2	Groundskeeping	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
98	Institution	7	25	2	Njala Corp	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
96	Institution	6	24	4	VNIIPAS-B	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
179	Location	12	45	3	Smart Meter	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
178	Location	12	45	2	Power Station	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
176	Location	11	44	4	Shelter	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
177	Location	12	45	1	Emergency Services	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
103	Institution	7	26	3	Mmeri	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
100	Institution	7	25	4	Shim Shi Li	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
213	Role	14	54	1	Networking	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
210	Role	14	53	2	Reliability	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
205	Role	13	52	1	Build	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
145	Location	10	37	1	Dealership	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
2	Distro	1	1	2	Faerie	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
124	Institution	8	31	4	Tenure	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
151	Location	10	38	3	Lab	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
245	Role	16	62	1	Checkpoint	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
174	Location	11	44	2	Dormitory	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
42	Distro	3	11	2	Khepri	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
67	Institution	5	17	2	Tsinghua University	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
32	Distro	2	8	4	Resurrection	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
182	Location	12	46	2	Foyer	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
89	Institution	6	23	1	CAC	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
184	Location	12	46	4	Service Room	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
208	Role	13	52	4	Test	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
247	Role	16	62	3	Dispatch	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
172	Location	11	43	4	Room	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
71	Institution	5	18	3	University of Tokyo	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
4	Distro	1	1	4	Utopia	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
239	Role	15	60	3	Content Creation	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
237	Role	15	60	1	Advertising	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
233	Role	15	59	1	Benefits	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
147	Location	10	37	3	Print Shop	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
56	Distro	4	14	4	Olympus	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
62	Distro	4	16	2	Genesis	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
60	Distro	4	15	4	Sanctuary	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
235	Role	15	59	3	Recruiting	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
34	Distro	3	9	2	Ghost	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
69	Institution	5	18	1	Cornell	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
14	Distro	1	4	2	Nebula	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
217	Role	14	55	1	AppSec	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
170	Location	11	43	2	Business Center	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
228	Role	15	57	4	Purchasing	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
53	Distro	4	14	1	Hakim	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
117	Institution	8	30	1	AncesTree	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
249	Role	16	63	1	Inspection	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
30	Distro	2	8	2	Icarus	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
244	Role	16	61	4	Technician	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
201	Role	13	51	1	Backend	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
5	Distro	1	2	1	Dreamwalker	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
101	Institution	7	26	1	ATI Pay	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
36	Distro	3	9	4	Renegade	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
196	Role	13	49	4	UI/UX	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
157	Location	10	40	1	Bus Stop	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
21	Distro	2	6	1	Epicurus	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
85	Institution	6	22	1	CISA	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
185	Location	12	47	1	City Services	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
186	Location	12	47	2	Fire Station	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
90	Institution	6	23	2	KISA	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
37	Distro	3	10	1	Infinity	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
25	Distro	2	7	1	Cyberpunk	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
189	Location	12	48	1	Callbox Access	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
137	Location	9	35	1	Floating Desk	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
158	Location	10	40	2	City Park	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
122	Institution	8	31	2	ConTRAC	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
154	Location	10	39	2	Nightclub	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
74	Institution	5	19	2	MIT	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
153	Location	10	39	1	Coffee Shop	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
18	Distro	2	5	2	Immortal	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
138	Location	9	35	2	Kitchenette	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
216	Role	14	54	4	Server Room	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
212	Role	14	53	4	Web Dev	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
102	Institution	7	26	2	CONCAS	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
141	Location	9	36	1	Assigned Desk	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
142	Location	9	36	2	Cubicle	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
97	Institution	7	25	1	Akala Brands	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
41	Distro	3	11	1	Emerald	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
86	Institution	6	22	2	FINEP Brasil	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
33	Distro	3	9	1	Flux	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
248	Role	16	62	4	Guard	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
64	Distro	4	16	4	Miracle	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
221	Role	14	56	1	Call Center	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
121	Institution	8	31	1	Automata	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
70	Institution	5	18	2	Donders Institute	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
49	Distro	4	13	1	Forbidden City	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
200	Role	13	50	4	Scheduling	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
118	Institution	8	30	2	Mod Central	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
94	Institution	6	24	2	ENISA	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
224	Role	14	56	4	Support Tech	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
17	Distro	2	5	1	Elysium	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
181	Location	12	46	1	Basement	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
146	Location	10	37	2	Newsstand	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
173	Location	11	44	1	Barracks	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
1	Distro	1	1	1	Aadvark	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
169	Location	11	43	1	Ballroom	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
57	Distro	4	15	1	Golem	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
61	Distro	4	16	1	Eternity	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
165	Location	11	42	1	Gated	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
232	Role	15	58	4	Secretary	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
10	Distro	1	3	2	Hydra	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
11	Distro	1	3	3	Serpent	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
9	Distro	1	3	1	Dragon	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
12	Distro	1	3	4	Wyvern	(1,Common,1)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Common,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Uncommon,0)	(1,Rare,0)	(1,Rare,0)	(1,Legendary,0)
\.


--
-- Data for Name: detail/general; Type: TABLE DATA; Schema: public; Owner: smithy
--

COPY public."detail/general" (id, name, kind) FROM stdin;
14	IT	Role
2	Fringe	Distro
15	People	Role
12	Unauthorized	Location
5	Academic	Institution
4	Restricted	Distro
3	Hardened	Distro
6	Bureaucratic	Institution
7	Corporate	Institution
11	Residence	Location
10	Public	Location
9	Office	Location
8	Decentralized	Institution
1	Consumer	Distro
13	Developer	Role
16	Physical	Role
\.


--
-- Data for Name: detail/specific; Type: TABLE DATA; Schema: public; Owner: smithy
--

COPY public."detail/specific" (id, name, general) FROM stdin;
1	Casual	1
2	Content	1
3	Gaming	1
4	Productivity	1
5	Exotic	2
6	Niche	2
7	Retro	2
8	Source	2
9	Anonymous	3
10	Crypto	3
11	Government	3
12	Industry	3
13	Access	4
14	Distribution	4
15	Install	4
16	Use	4
17	CompSci	5
18	Cybernetics	5
19	Engineering	5
20	Theoretical	5
21	Africa	6
22	Americas	6
23	Asia	6
24	EU	6
25	Consumer	7
26	Entertainment	7
27	Industrial	7
28	Military	7
29	Activist	8
30	Enthusiast	8
31	Freelance	8
32	OpenSource	8
33	Campus	9
34	Ephemeral	9
35	Satellite	9
36	Tower	9
37	Commercial	10
38	Education	10
39	Hospitality	10
40	Municipal	10
41	Apartment	11
42	Detached	11
43	Hotel	11
44	Shared	11
45	Infrastructure	12
46	Office	12
47	Public	12
48	Residential	12
49	Art	13
50	Production	13
51	Programming	13
52	QA	13
53	DevOps	14
54	Hardware	14
55	Security	14
56	Support	14
57	Accounting	15
58	Admin	15
59	HR	15
60	Marketing	15
61	Maintenance	16
62	Security	16
63	Supply	16
64	Trades	16
\.


--
-- Data for Name: mission; Type: TABLE DATA; Schema: public; Owner: smithy
--

COPY public.mission (id, mission_id, node_id, kind, state, north, east, south, west) FROM stdin;
2	1	2	Gateway	Known	\N	\N	(1,Open)	(3,Closed)
1	1	1	AccessPoint	Known	(2,Open)	(3,Closed)	\N	\N
3	1	3	Engine	Unknown	\N	(2,Closed)	\N	(1,Closed)
\.


--
-- Name: build/meta_2_id_seq; Type: SEQUENCE SET; Schema: public; Owner: smithy
--

SELECT pg_catalog.setval('public."build/meta_2_id_seq"', 16, true);


--
-- Name: build/meta_3_id_seq; Type: SEQUENCE SET; Schema: public; Owner: smithy
--

SELECT pg_catalog.setval('public."build/meta_3_id_seq"', 4, true);


--
-- Name: build_id_seq; Type: SEQUENCE SET; Schema: public; Owner: smithy
--

SELECT pg_catalog.setval('public.build_id_seq', 256, true);


--
-- Name: card_id_seq; Type: SEQUENCE SET; Schema: public; Owner: smithy
--

SELECT pg_catalog.setval('public.card_id_seq', 16, true);


--
-- Name: category_id_seq; Type: SEQUENCE SET; Schema: public; Owner: smithy
--

SELECT pg_catalog.setval('public.category_id_seq', 256, true);


--
-- Name: detail/meta_2_id_seq; Type: SEQUENCE SET; Schema: public; Owner: smithy
--

SELECT pg_catalog.setval('public."detail/meta_2_id_seq"', 16, true);


--
-- Name: detail/meta_3_id_seq; Type: SEQUENCE SET; Schema: public; Owner: smithy
--

SELECT pg_catalog.setval('public."detail/meta_3_id_seq"', 64, true);


--
-- Name: mission_id_seq; Type: SEQUENCE SET; Schema: public; Owner: smithy
--

SELECT pg_catalog.setval('public.mission_id_seq', 3, true);


--
-- Name: build/company build/meta_2_pk; Type: CONSTRAINT; Schema: public; Owner: smithy
--

ALTER TABLE ONLY public."build/company"
    ADD CONSTRAINT "build/meta_2_pk" PRIMARY KEY (id);


--
-- Name: build/market build/meta_3_pk; Type: CONSTRAINT; Schema: public; Owner: smithy
--

ALTER TABLE ONLY public."build/market"
    ADD CONSTRAINT "build/meta_3_pk" PRIMARY KEY (id);


--
-- Name: detail/general detail/meta_2_pk; Type: CONSTRAINT; Schema: public; Owner: smithy
--

ALTER TABLE ONLY public."detail/general"
    ADD CONSTRAINT "detail/meta_2_pk" PRIMARY KEY (id);


--
-- Name: detail/specific detail/meta_3_pk; Type: CONSTRAINT; Schema: public; Owner: smithy
--

ALTER TABLE ONLY public."detail/specific"
    ADD CONSTRAINT "detail/meta_3_pk" PRIMARY KEY (id);


--
-- Name: detail detail_pk; Type: CONSTRAINT; Schema: public; Owner: smithy
--

ALTER TABLE ONLY public.detail
    ADD CONSTRAINT detail_pk PRIMARY KEY (id);


--
-- Name: mission id; Type: CONSTRAINT; Schema: public; Owner: smithy
--

ALTER TABLE ONLY public.mission
    ADD CONSTRAINT id PRIMARY KEY (id);


--
-- Name: build build_2_fk; Type: FK CONSTRAINT; Schema: public; Owner: smithy
--

ALTER TABLE ONLY public.build
    ADD CONSTRAINT build_2_fk FOREIGN KEY (company) REFERENCES public."build/company"(id);


--
-- Name: build build_3_fk; Type: FK CONSTRAINT; Schema: public; Owner: smithy
--

ALTER TABLE ONLY public.build
    ADD CONSTRAINT build_3_fk FOREIGN KEY (market) REFERENCES public."build/market"(id);


--
-- Name: detail/specific detail/meta_3_2_fk; Type: FK CONSTRAINT; Schema: public; Owner: smithy
--

ALTER TABLE ONLY public."detail/specific"
    ADD CONSTRAINT "detail/meta_3_2_fk" FOREIGN KEY (general) REFERENCES public."detail/general"(id);


--
-- Name: detail detail_2_fk; Type: FK CONSTRAINT; Schema: public; Owner: smithy
--

ALTER TABLE ONLY public.detail
    ADD CONSTRAINT detail_2_fk FOREIGN KEY (general) REFERENCES public."detail/general"(id);


--
-- Name: detail detail_3_fk; Type: FK CONSTRAINT; Schema: public; Owner: smithy
--

ALTER TABLE ONLY public.detail
    ADD CONSTRAINT detail_3_fk FOREIGN KEY (specific) REFERENCES public."detail/specific"(id);


--
-- PostgreSQL database dump complete
--

