[
	.Images[] | select(
		.Tags[] | select(
			.Key == "Name" and .Value == "zoolander"
		)
	) 
	| {ImageId: .ImageId, CreationDate: .CreationDate}
]
| sort_by(.CreationDate) # Order by age
| .[0:length - 1]        # Grab the n-1 oldest
| .[].ImageId            # Spit out their ids
