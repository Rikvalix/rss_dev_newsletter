-- Add up migration script here
create table ai_classification_feed_items
(
    ai_classification_id integer,
    feed_item_id integer,
    constraint pk_ai_classification_feed_items primary key (ai_classification_id,feed_item_id),
    constraint fk_ai_classification foreign key (ai_classification_id) references ai_classification(id),
    constraint fk_feed_items foreign key (feed_item_id) references feed_items(id)
);